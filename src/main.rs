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
    create_default_image::<fractals::Cube>(ColorType::Normal);
    create_image(
        fractals::Cube::new(Point::new(0., 3., 0.), Point::ZERO, Point::ONE, 3.0),
        "-2",
        ColorType::Normal,
    );
    create_image(
        fractals::Cube::new(Point::new(0.2, 0.2, 0.2), Point::ZERO, Point::ONE, 3.0),
        "-3",
        ColorType::Normal,
    );
    create_image(
        fractals::Cube::new(Point::ZERO, Point::ZERO, Point::new(1.2, 1.0, 0.4), 3.0),
        "-4",
        ColorType::Grayscale,
    );

    create_default_image::<fractals::Spheres>(ColorType::Normal);
    create_image(fractals::Spheres::new(0.1), "-2", ColorType::Grayscale);

    create_default_image::<fractals::Tetrahedron>(ColorType::Normal);
    create_image(
        fractals::Tetrahedron::new(Point::new(4.0, 4., 0.0), Point::ZERO),
        "-2",
        ColorType::Distance,
    );
    create_image(
        fractals::Tetrahedron::new(Point::new(0.0, -0.2, 0.0), Point::ZERO),
        "-3",
        ColorType::Grayscale,
    );
    create_image(
        fractals::Tetrahedron::new(Point::new(0.0, 0.35, 0.0), Point::new(0., -0.2, 0.)),
        "-4",
        ColorType::Normal,
    );
}

fn create_default_image<T: fractals::Fractal + Default>(color_type: ColorType) {
    create_image(T::default(), "-1", color_type)
}

fn create_image<T: fractals::Fractal>(fractal: T, config_name: &str, color_type: ColorType) {
    let filename = format!(
        "fractal-{}{}-{:?}.png",
        std::any::type_name::<T>().split(':').last().unwrap(),
        config_name,
        color_type,
    )
    .to_lowercase();
    println!("Creating {}", filename);
    let instant = Instant::now();

    let width = WIDTH * SSAA;
    let height = HEIGHT * SSAA;
    let screen_dim = Point::new(width as f64, height as f64, 0.0);
    let mut img = vec![Rgb([0u8; 3]); (width as usize) * (height as usize)];

    img.par_iter_mut().enumerate().for_each(|(n, pixel)| {
        let y = (n as u32) / width;
        let x = (n as u32) - (y * width);
        *pixel = fractal.render(Point::new(x as f64, y as f64, 0.0), screen_dim, color_type);
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
