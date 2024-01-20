use image::RgbImage;
use ndarray::Array3;
use std::fs;
use indicatif::ProgressBar;
use nalgebra::Vector3;
use ray::Ray;
mod ray;

fn array_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

fn get_color(ray: Ray) -> Vector3<u8> {
    Vector3::new(100,200,255)
}

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f32 * aspect_ratio) as usize;
    let view_width : f32 = 2.0;
    let view_height: f32 = view_width * (image_height as f32 / image_width as f32);

    let focal_length: f32 = 1.0;
    let camera_point = Vector3::new(0.0,0.0,0.0);

    let viewport_u = Vector3::new(view_width,0.0,0.0);
    let viewport_v = Vector3::new(0.0,-view_height,0.0);

    let pixel_delta_u = viewport_u / (image_width as f32);
    let pixel_delta_v = viewport_v / (image_height as f32);

    let view_upper_left = camera_point - Vector3::new(0.0,0.0,focal_length) - viewport_u/2.0 - viewport_v/2.0;

    let pixel00_loc = view_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);



    let mut pixels: Array3<u8> = Array3::zeros((image_width, image_height, 3));
    let bar  = ProgressBar::new((image_width * image_height * 3) as u64);
    for x in 0..image_width {
        for y in 0..image_height {
            bar.inc(1);
            let pixel_center = pixel00_loc + (x as f32 * pixel_delta_u) + (y as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_point;
            let r = Ray::new(camera_point,ray_direction);
            let color = get_color(r);
            let (r,g,b) = (color[0], color[1], color[2]);
            pixels[[x,y,0]] = r;
            pixels[[x,y,1]] = g;
            pixels[[x,y,2]] = b;
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
