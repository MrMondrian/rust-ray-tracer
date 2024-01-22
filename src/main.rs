use image::RgbImage;
use ndarray::Array3;
use std::fs;
use indicatif::ProgressBar;
use nalgebra::Vector3;
use ray::Ray;
mod ray;

fn hit_sphere(center: Vector3<f64>, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let half_b = oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn array_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

fn get_color(ray: Ray) -> Vector3<f64> {
    let t = hit_sphere(Vector3::new(0.0,0.0,-1.0), 0.5, &ray);
    if t > 0.0 {
        let n = ray.at(t) - Vector3::new(0.0,0.0,-1.0);
        let normed = n / n.norm();
        return 0.5 * Vector3::new(normed[0] + 1.0, normed[1] + 1.0, normed[2] + 1.0);
    }
    let direction = ray.direction / ray.direction.norm();
    let a = 0.5 * (direction[1] + 1.0);
    return (1.0-a)*Vector3::new(1.0, 1.0, 1.0) + a*Vector3::new(0.5, 0.7, 1.0);
}

fn float_pixel_to_byte(pixel: &f64) -> u8 {
    let scaled = pixel * 255.999;
    if scaled > 255.999 {
        return 255 
    }
    else {
        return scaled as u8;
    }
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let view_height : f64 = 2.0;
    let view_width: f64 = view_height * (image_width as f64 / image_height as f64);

    let focal_length: f64 = 1.0;
    let camera_point = Vector3::new(0.0,0.0,0.0);

    let viewport_u = Vector3::new(view_width,0.0,0.0);
    let viewport_v = Vector3::new(0.0,-view_height,0.0);

    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    let view_upper_left = camera_point - Vector3::new(0.0,0.0,focal_length) - viewport_u/2.0 - viewport_v/2.0;

    let pixel00_loc = view_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);



    let mut pixels: Array3<u8> = Array3::zeros((image_height, image_width, 3));
    let bar  = ProgressBar::new((image_width * image_height * 3) as u64);
    for j in 0..image_height {
        for i in 0..image_width {
            bar.inc(1);
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_point;
            let r = Ray::new(camera_point,ray_direction);
            let color = get_color(r);
            let (r,g,b) = (color[0], color[1], color[2]);
            pixels[[j,i,0]] = float_pixel_to_byte(&r);
            pixels[[j,i,1]] = float_pixel_to_byte(&g);
            pixels[[j,i,2]] = float_pixel_to_byte(&b);
        }
    }
    bar.finish();
    let image = array_to_image(pixels);
    fs::remove_file("out.png").unwrap_or_default();
    let result = image.save("out.png");
    match result {
        Ok(_) => println!("Image saved successfully"),
        Err(e) => println!("Error saving image: {}", e),
    }
}
