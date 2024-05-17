use nalgebra::Vector3;
use rand::prelude::*;
use image::RgbImage;
use ndarray::Array3;
use std::fs;
use indicatif::ProgressBar;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::hitable_list::HitableList;



pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    image_height: usize,
    center: Vector3<f64>,
    pixel00_loc: Vector3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    samples_per_pixel: usize,
    pixels_sample_scale: f64,
    rng: ThreadRng,
    max_depth: usize,
}

impl Camera {

    pub fn new() -> Self {
        let aspect_ratio = 16.0/9.0;
        let image_width = 400;
        let focal_length: f64 = 1.0;
        let view_height : f64 = 2.0;
        let center = Vector3::new(0.0,0.0,0.0);
        let samples_per_pixel = 50;
        let max_depth = 10;

        let pixels_sample_scale = 1.0 / (samples_per_pixel as f64);
        let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
        let view_width: f64 = view_height * (image_width as f64 / image_height as f64);


        let viewport_u = Vector3::new(view_width,0.0,0.0);
        let viewport_v = Vector3::new(0.0,-view_height,0.0);

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let view_upper_left = center - Vector3::new(0.0,0.0,focal_length) - viewport_u/2.0 - viewport_v/2.0;

        let pixel00_loc = view_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let rng = rand::thread_rng();
        Self{aspect_ratio, image_width, image_height, center, pixel00_loc, pixel_delta_u, pixel_delta_v, samples_per_pixel, pixels_sample_scale, rng, max_depth}
    }

    fn get_color(&mut self, ray: Ray, world: &HitableList, depth: usize) -> Vector3<f64> {
        if depth <= 0 {
            return Vector3::new(0.0,0.0,0.0);
        }
        if let Some(x) = world.hit(&ray, Interval::new(0.001, f64::INFINITY)) {
            // is this line needed?
            let new_direction = x.normal + self.random_unit_vector();
            let new_ray = Ray::new(x.p, new_direction);
            return 0.5 * (self.get_color(new_ray,world, depth -1));
        }
        let direction = ray.direction / ray.direction.norm();
        let a = 0.5 * (direction[1] + 1.0);
        return (1.0-a)*Vector3::new(1.0, 1.0, 1.0) + a*Vector3::new(0.5, 0.7, 1.0);
    }

    pub fn render(&mut self, world: &HitableList) -> () {

        let mut pixels: Array3<u8> = Array3::zeros((self.image_height, self.image_width, 3));
        let bar  = ProgressBar::new((self.image_width * self.image_height) as u64);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                bar.inc(1);
                let mut pixel_color = Vector3::new(0.0,0.0,0.0);
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i as f64,j as f64);
                    pixel_color += self.get_color(ray, world, self.max_depth);
                }
                pixel_color *= self.pixels_sample_scale;
                pixels[[j,i,0]] = Camera::float_pixel_to_byte(&pixel_color.x);
                pixels[[j,i,1]] = Camera::float_pixel_to_byte(&pixel_color.y);
                pixels[[j,i,2]] = Camera::float_pixel_to_byte(&pixel_color.z);
            }
        }
        bar.finish();
        let image = Camera::array_to_image(pixels);
        fs::remove_file("out.png").unwrap_or_default();
        let result = image.save("out.png");
        match result {
            Ok(_) => println!("Image saved successfully"),
            Err(e) => println!("Error saving image: {}", e),
        }
    }

    fn array_to_image(arr: Array3<u8>) -> RgbImage {
        assert!(arr.is_standard_layout());

        let (height, width, _) = arr.dim();
        let raw = arr.into_raw_vec();

        RgbImage::from_raw(width as u32, height as u32, raw)
            .expect("container should have the right size for the image dimensions")
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

    fn get_ray(&mut self, i: f64, j: f64) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc + ((i + offset.x) * self.pixel_delta_u) + ((j + offset.y) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        return Ray::new(ray_origin, ray_direction);
    }

    fn sample_square(&mut self) -> Vector3<f64> {
        Vector3::new(self.rng.gen::<f64>() - 0.5, self.rng.gen::<f64>() - 0.5, 0.0)
    } 

    fn random_vector(&mut self) -> Vector3<f64> {
        return Vector3::new(self.rng.gen_range(-1.0..1.0),self.rng.gen_range(-1.0..1.0),self.rng.gen_range(-1.0..1.0))
    }

    fn random_unit_vector(&mut self) -> Vector3<f64> {
        let vec = self.random_vector();
        return vec / vec.norm();
    }
}
