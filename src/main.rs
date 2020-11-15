extern crate image;
use image::{ImageBuffer};

mod vec3;
mod sdf;
use sdf::SDF;

fn main() {
    let width = 960;
    let height = 540;
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        SDF::render(
            (x as f32 - width as f32 / 2.0) / height as f32,
            (y as f32 - height as f32 / 2.0) / height as f32,
        )
    });

    img.save("scene.png").unwrap();
}
