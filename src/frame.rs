use image::{GenericImageView, RgbImage, Rgb as ImageRgb, Pixel};
use std::path::{ Path };

#[derive(Copy, Clone)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

struct MathShape {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    base_color: Rgb,
    gradient: (i32, i32, i32),
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
    let x = matrix.len();
    let y = if x > 0 { matrix[0].len()} else { 0 };
    
    println!("Compressing image using pattern based clustering and reduction");
    
    let mut visited = vec![vec![false; y]; x];
    let mut shapes: Vec<MathShape> = Vec::new();

    for r in 0..x {
        for c in 0..y {
            if visited[r][c] { continue; }

            let start_pixel = matrix[r][c];
            let mut width = 1;
            
            if c + 1 < y {
                let next_pixel = matrix[r][c + 1];
                let dr = next_pixel.r as i32 - start_pixel.r as i32;
                let dg = next_pixel.g as i32 - start_pixel.g as i32;
                let db = next_pixel.b as i32 - start_pixel.b as i32;

                while c + width < y && !visited[r][c + width] {
                    let curr = matrix[r][c + width];
                    let prev = matrix[r][c + width - 1];
                    
                    if (curr.r as i32 - prev.r as i32) == dr &&
                       (curr.g as i32 - prev.g as i32) == dg &&
                       (curr.b as i32 - prev.b as i32) == db {
                        width += 1;
                    } else {
                        break;
                    }
                }

                for i in 0..width { visited[r][c + i] = true; }
                
                shapes.push(MathShape {
                    x: c as u32,
                    y: r as u32,
                    width: width as u32,
                    height: 1,
                    base_color: start_pixel,
                    gradient: (dr, dg, db),
                });

                println!("Found somthing at, {} {}", c, r)
            }
        }
    }

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
