pub mod frame;
fn main() {
    println!("yeeyyeyeyey");
    let img = frame::image_to_matrix("/home/ekah/save.png").expect("error path");
    frame::display_color_terminal(&img);
}
