use image::{GenericImageView, RgbImage, Rgb as ImageRgb, Pixel};
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
    let y = if x > 0 { matrix[0].len() as u32 } else { 0 };

    println!("Compressing image using ")   
}

fn matrix_to_image<P: AsRef<Path>>(matrix: &Matrix, path: P) -> Result<(), image::ImageError> {
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

use terminal_size::{Width, Height, terminal_size};

fn get_auto_downsample_factor(image_width: usize) -> usize {
    if let Some((Width(w), Height(_h))) = terminal_size() {
        let terminal_width = w as usize;
        
        let available_space = terminal_width / 3;

        if image_width > available_space {
            return (image_width / available_space).max(1);
        }
    }
    
    1
}

fn display_color_terminal(matrix: &Matrix) {
    let factor = get_auto_downsample_factor(if !matrix.is_empty() { matrix[0].len()} else { 0 });

    let old_height = matrix.len();
    let old_width = if old_height > 0 { matrix[0].len() } else { 0 };

    let new_height = old_height / factor;
    let new_width = old_width / factor;

    let mut new_matrix = vec![vec![Rgb { r: 0, g: 0, b: 0 }; new_width]; new_height];

    for y in 0..new_height {
        for x in 0..new_width {
            let mut r_sum: u32 = 0;
            let mut g_sum: u32 = 0;
            let mut b_sum: u32 = 0;
            let count = (factor * factor) as u32;

            // Sum up all pixels in the 'factor x factor' block
            for fy in 0..factor {
                for fx in 0..factor {
                    let pixel = &matrix[y * factor + fy][x * factor + fx];
                    r_sum += pixel.r as u32;
                    g_sum += pixel.g as u32;
                    b_sum += pixel.b as u32;
                }
            }

            // Calculate the average
            new_matrix[y][x] = Rgb {
                r: (r_sum / count) as u8,
                g: (g_sum / count) as u8,
                b: (b_sum / count) as u8,
            };
        }
    }

    for row in new_matrix {
        for pixel in row {
            print!("\x1b[48;2;{};{};{}m  ", pixel.r, pixel.g, pixel.b);
        }
        println!("\x1b[0m");
    }
}

fn main() {
    println!("yeeyyeyeyey");
    let img = image_to_matrix("/home/ekah/save.png").expect("error path");
    display_color_terminal(&img);
}
