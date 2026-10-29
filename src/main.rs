use minifb::{Key, Window, WindowOptions};

fn main() {
    let width = 640;
    let height = 480;

    // Create a pixel buffer filled with dark grey (0x00222222)
    let buffer: Vec<u32> = vec![0x00222222; width * height];

    // Create the window
    let mut window = Window::new(
        "Simple Minifb Window",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Failed to create window: {}", e);
    });

    // Main window loop - runs until you close it or press Escape
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Redraw the screen buffer and handle window events
        window
            .update_with_buffer(&buffer, width, height)
            .unwrap();
    }
}
