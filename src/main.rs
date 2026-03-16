use image::{GenericImageView, DynamicImage, Pixel};
use std::path::Path;

#[derive(Copy, Clone)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

type Matrix = Vec<Vec<Rgb>>;

fn image_to_matrix<P: AsRef<Path>>(path: P) -> Result<Matrix, image::ImageError> {
    let img = image::open(path)?;
    let (width, height) = img.dimensions();

    let mut matrix = vec![vec![Rgb { r: 0, g: 0, b: 0 }; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgb = pixel.to_rgb();
            matrix[y as usize][x as usize] = Rgb {
                r: rgb[0],
                g: rgb[1],
                b: rgb[2],
            };
        }
    }

    Ok(matrix)
}

fn image_compresso(matrix: &mut Matrix) {
    let x = matrix.len() as u32;
    let y = matrix[0].len() as u32;

}

fn main() {
    println!("yeeyyeyeyey");
}
