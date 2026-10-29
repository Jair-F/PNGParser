
mod ui;
mod png;


fn main() {
    let png_file = "img/hand_800x800.png";

    let tmp = png::read::read_png(&png_file).expect("Error parsing the png");
    println!("{tmp:#X?}");

    let tmp = png::PNG::from_file(&png_file);

    // ui::show_window();
}
