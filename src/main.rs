pub mod frame;

use std::io;
use crate::frame::image_to_matrix;


fn main() {
    let mat = frame::image_to_matrix("/home/ekah/save.png").expect("fail");
    frame::matrix_to_terminal_show_thingy(&mat);

}