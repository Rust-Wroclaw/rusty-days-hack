mod fractals;
mod point;

use image::imageops::FilterType;
use image::DynamicImage;

fn main() {
    create_image::<fractals::Spheres>();
    create_image::<fractals::Triangles>();
    create_image::<fractals::TrianglesWithFold>();
}

fn create_image<T: fractals::Fractal>() {
    let width = 1920;
    let height = 1080;
    let ssaa = 2;

    let filename = format!(
        "fractal-{}.png",
        std::any::type_name::<T>().split(':').last().unwrap()
    );
    println!("Creating {}", filename);
    let img_buffer = T::img(width * ssaa, height * ssaa);
    DynamicImage::ImageRgb8(img_buffer)
        .resize(width, height, FilterType::Lanczos3)
        .save(&filename)
        .unwrap();
}
