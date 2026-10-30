use minifb::{Key, Window, WindowOptions};

use crate::png::Png;

pub fn show_window(image: Png) {
    let width = image.ihdr.width as usize;
    let height = image.ihdr.height as usize;

    // Create a pixel buffer filled with dark grey (0x00222222)
    let buffer: Vec<u32> = vec![0x00222222; width * height];

    let mut window = Window::new("PNG Parser", width, height, WindowOptions::default()).unwrap();
    window.set_target_fps(30);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
