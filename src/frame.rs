use image::{GenericImageView, RgbImage, Rgb as ImageRgb, Pixel};
use std::path::{ Path, PathBuf };
use std::fs::File;
use std::io::{ self, Write, BufWriter };

#[derive(Copy, Clone)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

struct LineForm {
    x1: u32,
    y1: u32,
    x2: u32,
    m: u32,
    base_color: Rgb,
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

pub fn image_compresso<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let matrix = image_to_matrix(&path).expect("error converting image to matrix");
    let x = matrix.len();
    let y = if x > 0 { matrix[0].len()} else { 0 };
    
    println!("Compressing image using pattern based clustering and reduction");
    
    let mut visited = vec![vec![false; y]; x];
    let mut if_line: Vec<LineForm> = Vec::new();

    // for r in 0..x {
    //     for c in 0..y {
    //         if visited[r][c] { continue; }

    //         let start_pixel = matrix[r][c];
    //         let mut width = 1;
            
    //         if c + 1 < y {
    //             let next_pixel = matrix[r][c + 1];
    //             let dr = next_pixel.r as i32 - start_pixel.r as i32;
    //             let dg = next_pixel.g as i32 - start_pixel.g as i32;
    //             let db = next_pixel.b as i32 - start_pixel.b as i32;

    //             while c + width < y && !visited[r][c + width] {
    //                 let curr = matrix[r][c + width];
    //                 let prev = matrix[r][c + width - 1];
                    
    //                 if (curr.r as i32 - prev.r as i32) == dr &&
    //                    (curr.g as i32 - prev.g as i32) == dg &&
    //                    (curr.b as i32 - prev.b as i32) == db {
    //                     width += 1;
    //                 } else {
    //                     break;
    //                 }
    //             }

    //             for i in 0..width { visited[r][c + i] = true; }
                
    //             shapes.push(MathShape {
    //                 x: c as u32,
    //                 y: r as u32,
    //                 width: width as u32,
    //                 height: 1,
    //                 base_color: start_pixel,
    //                 gradient: (dr, dg, db),
    //             });

    //             println!("Found somthing at, {} {}", c, r)
    //         }
    //     }
    // }

    write_matrix_binary(matrix, &path)
}

fn write_matrix_binary<P: AsRef<Path>>(matrix: Matrix, path: P) -> io::Result<()> {
    let out_path: PathBuf = path.as_ref().with_extension("cnan");

    let height = matrix.len() as u32;
    let width = if height > 0 { matrix[0].len() as u32 } else { 0 };

    let file = File::create(&out_path)?;
    let mut writer = BufWriter::with_capacity(8 + (width as usize * height as usize * 3), file);

    writer.write_all(&width.to_le_bytes())?;
    writer.write_all(&height.to_le_bytes())?;

    for row in matrix.iter() {
        for px in row.iter() {
            writer.write_all(&[px.r, px.g, px.b])?;
        }
    }

    writer.flush()?;
    println!("Wrote compressed matrix to {}", out_path.display());
    Ok(())
}