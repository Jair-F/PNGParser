use minifb::{Key, Window, WindowOptions};

mod parser;

fn show_window() {
    let width = 800;
    let height = 800;

    // Create a pixel buffer filled with dark grey (0x00222222)
    let buffer: Vec<u32> = vec![0x00222222; width * height];

    let mut window = Window::new("PNG Parser", width, height, WindowOptions::default()).unwrap();
    window.set_target_fps(30);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

fn main() {
    let tmp = parser::read::read_png("img/hand_800x800.png").expect("Error parsing the png");
    println!("{tmp:#X?}")

    // let png_bytes:Vec<u8> = parser::read::read_file().unwrap();
    // println!("{png_bytes:02X?}");

    // show_window();
}
