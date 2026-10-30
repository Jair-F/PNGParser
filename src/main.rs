mod png;
mod ui;

fn main() {
    let png_file = "img/hand_800x800.png";
    let image = png::Png::from_file(&png_file).unwrap();

    print!("Read the following char types: ");
    for chunk in &image.chunks {
        let out = &chunk.chunk_type_str;
        print!("{out}, ");
    }
    println!();
    // println!("PNG image: {image:#?}");

    // ui::draw::show_window();
}
