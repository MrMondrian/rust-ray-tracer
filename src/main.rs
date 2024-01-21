use image::RgbImage;
use ndarray::Array3;
use std::fs;
use indicatif::ProgressBar;
use nalgebra::Vector3;
use ray::Ray;
mod ray;

fn hit_sphere(center: Vector3<f64>, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    return discriminant >= 0.0;
}

fn array_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

fn get_color(ray: Ray) -> Vector3<u8> {
    if hit_sphere(Vector3::new(0.0,0.0,-1.0), 0.5, &ray) {
        return Vector3::new(255,0,0);
    }
    let direction = ray.direction / ray.direction.norm();
    let a = 0.5 * (direction[1] + 1.0);
    let new_color = (1.0-a)*Vector3::new(1.0, 1.0, 1.0) + a*Vector3::new(0.5, 0.7, 1.0);
    return float_color_to_byte_color(new_color)
}

fn float_color_to_byte_color(color: Vector3<f64>) -> Vector3<u8> {
    let result = color
    .iter()
    .map(|x| float_pixel_to_byte(x));
    return Vector3::from_iterator(result);
}

fn float_pixel_to_byte(pixel: &f64) -> u8 {
    let scaled = pixel * 255.0;
    if scaled > 255.0 {
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
            pixels[[j,i,0]] = r;
            pixels[[j,i,1]] = g;
            pixels[[j,i,2]] = b;
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
