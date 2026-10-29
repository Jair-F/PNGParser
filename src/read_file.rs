use std::fs;

pub fn read_file() -> std::io::Result<Vec<u8>> {
    // Reads the raw bytes directly
    let bytes: Vec<u8> = fs::read("./img/hand_800x800.png")?;
    
    println!("Read {} bytes successfully.", bytes.len());
    Ok(bytes)
}

