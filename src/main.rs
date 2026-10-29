mod png;
mod ui;

fn main() {
    let png_file = "img/hand_800x800.png";

    let tmp = png::helpers::load_file_without_header(&png_file).expect("Error parsing the png");
    println!("{tmp:X?}");

    let _tmp = png::PNG::from_file(&png_file);

    ui::draw::show_window();
}
