use image::{GenericImageView, RgbImage, Rgb as ImageRgb, Pixel};
use std::path::{ Path };

#[derive(Copy, Clone)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

type Matrix = Vec<Vec<Rgb>>;

pub fn image_to_matrix<P: AsRef<Path>>(path: P) -> Result<Matrix, image::ImageError> {
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

pub fn image_compresso(matrix: &mut Matrix) {
    let x = matrix.len() as u32;
    let y = if x > 0 { matrix[0].len() as u32 } else { 0 };
    
    println!("Compressing image using pattern clustering and reduction")
}

pub fn matrix_to_image<P: AsRef<Path>>(matrix: &Matrix, path: P) -> Result<(), image::ImageError> {
    let y = matrix.len() as u32;
    let x = if y > 0 { matrix[0].len() as u32 } else { 0 };

    let mut imgbuf = RgbImage::new(x, y);

    for (yy, row) in matrix.iter().enumerate() {
        for (xx, pixel) in row.iter().enumerate() {
            let image_pixel = ImageRgb([pixel.r, pixel.g, pixel.b]);
            imgbuf.put_pixel(xx as u32, yy as u32, image_pixel);
        }
    }

    imgbuf.save(path)?;

    Ok(())
}
