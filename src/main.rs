mod fractals;
mod point;

use crate::fractals::ColorType;
use crate::point::Point;
use image::imageops::FilterType;
use image::{DynamicImage, ImageBuffer, Rgb};
use rayon::prelude::*;
use std::time::Instant;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const SSAA: u32 = 2;

fn main() {
    create_images::<fractals::Spheres>();
    create_images::<fractals::Triangles>();
    create_images::<fractals::Squares>();
}

fn create_images<T: fractals::Fractal>() {
    for color_type in &[
        ColorType::BlackAndWhite,
        ColorType::ColorByDistanceValue,
        ColorType::ColorDiffuse,
    ] {
        create_image::<T>(*color_type)
    }
}

fn create_image<T: fractals::Fractal>(color_type: ColorType) {
    let filename = format!(
        "fractal-{}-{:?}.png",
        std::any::type_name::<T>().split(':').last().unwrap(),
        color_type
    );
    println!("Creating {}", filename);
    let instant = Instant::now();

    let width = WIDTH * SSAA;
    let height = HEIGHT * SSAA;
    let screen_dim = Point::new(width as f64, height as f64, 0.0);
    let mut img = vec![Rgb([0u8; 3]); (width as usize) * (height as usize)];

    img.par_iter_mut().enumerate().for_each(|(n, pixel)| {
        let y = (n as u32) / width;
        let x = (n as u32) - (y * width);
        *pixel = T::render(Point::new(x as f64, y as f64, 0.0), screen_dim, color_type);
    });

    println!("Completed image in {:?} ms", instant.elapsed().as_millis());

    let mut buf = Vec::with_capacity(img.len() * 3);
    img.into_iter()
        .for_each(|pixel| buf.extend_from_slice(&pixel.0));
    let img_buffer = ImageBuffer::from_vec(width, height, buf).unwrap();
    DynamicImage::ImageRgb8(img_buffer)
        .resize(WIDTH, HEIGHT, FilterType::Lanczos3)
        .save(&filename)
        .unwrap();
}
