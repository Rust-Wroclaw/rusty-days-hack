mod fractals;
mod point;

use crate::fractals::ColorType;
use image::imageops::FilterType;
use image::DynamicImage;
use std::time::Instant;

fn main() {
    create_images::<fractals::Spheres>();
    create_images::<fractals::Triangles>();
    create_images::<fractals::TrianglesWithFold>();
}

fn create_images<T: fractals::Fractal>() {
    let width = 1920;
    let height = 1080;
    let ssaa = 2;

    for color_type in &[
        ColorType::BlackAndWhite,
        ColorType::Colored,
        ColorType::ColoredWithShades,
    ] {
        let fractal_name = std::any::type_name::<T>().split(':').last().unwrap();
        let filename = format!("fractal-{}-{:?}.png", fractal_name, color_type,);
        println!("Creating {}", filename);
        let instant = Instant::now();

        let img_buffer = T::img(width * ssaa, height * ssaa, color_type);
        println!(
            "Completed image for {} - {:?} in {:?} ms",
            fractal_name,
            color_type,
            instant.elapsed().as_millis()
        );
        DynamicImage::ImageRgb8(img_buffer)
            .resize(width, height, FilterType::Lanczos3)
            .save(&filename)
            .unwrap();
    }
}
