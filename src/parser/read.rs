use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::fs;

use crate::parser::interfaces;

pub fn read_file() -> std::io::Result<Vec<u8>> {
    // Reads the raw bytes directly
    let bytes: Vec<u8> = fs::read("./img/hand_800x800.png")?;

    println!("Read {} bytes successfully.", bytes.len());
    Ok(bytes)
}

fn valid_png_signature(png_bytes: &[u8; 8]) -> bool {
    if png_bytes.len() != 8 {
        return false;
    }
    let png_signature: u64 = 0x89504E470D0A1A0A;
    let mut file_sig: u64 = 0;

    for i in 0..8 {
        file_sig = file_sig << 8;
        file_sig += png_bytes[i] as u64;
    }

    file_sig == png_signature
}

fn read_file_header(file_path: &str) -> Result<[u8; 8], Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut file_header: [u8; 8] = [0x00; 8];
    reader.read_exact(&mut file_header)?;

    return Ok(file_header);
}

pub fn read_png(file_path: &str) -> Result<interfaces::Chunk, Box<dyn std::error::Error>> {
    let file_header = read_file_header(&file_path).expect("Failed to read file_header");
    if !valid_png_signature(&file_header) {
        return Err("Not a png file".into());
    }

    let mut file = File::open(file_path)?;
    file.seek(SeekFrom::Start(8))?;
    let mut reader = BufReader::new(file);
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).err();

    let mut chunk_type: u32 = 0;
    for i in 0..3 {
        chunk_type = chunk_type << 8;
        chunk_type += bytes[i] as u32;
    }

    println!("chunk type: 0x{chunk_type:02x?}");

    let data = &bytes[3..10].to_vec();

    let c = interfaces::Chunk::new(chunk_type, data.clone());

    Ok(c)
}
