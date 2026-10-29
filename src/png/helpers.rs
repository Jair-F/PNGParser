use crate::png::Chunk;

use super::constants;
use super::interfaces;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Seek, SeekFrom};

fn u8_to_u32_msb(data_bytes: &[u8; 4]) -> u32 {
    let mut ret: u32 = 0;
    for i in 0..4 {
        ret = ret << 8;
        ret |= data_bytes[i] as u32;
    }
    ret
}

fn valid_png_signature(png_bytes: &[u8; 8]) -> bool {
    if png_bytes.len() != 8 {
        return false;
    }
    let mut signature: u64 = (u8_to_u32_msb(&png_bytes[0..4].try_into().unwrap()) as u64) << 32;
    signature += u8_to_u32_msb(&png_bytes[4..8].try_into().unwrap()) as u64;

    signature == constants::PNG_FILE_MAGIC_HEADER
}

fn read_file_header(file_path: &str) -> Result<[u8; 8], Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut file_header: [u8; 8] = [0x00; 8];
    reader.read_exact(&mut file_header)?;

    return Ok(file_header);
}

pub fn is_valid_png_file_header(file_path: &str) -> bool {
    let file_header = read_file_header(&file_path).expect("Failed to read file_header");
    valid_png_signature(&file_header)
}

fn parse_next_chunk(data_bytes: &mut Vec<u8>) -> io::Result<interfaces::Chunk> {
    if data_bytes.len() < 8 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "len of data bytes to short",
        ));
    }

    let chunk_size: u32 = u8_to_u32_msb(&data_bytes[0..4].try_into().unwrap());
    let chunk_type: u32 = u8_to_u32_msb(&data_bytes[4..8].try_into().unwrap());
    let data_end_pos = 8 + chunk_size as usize;

    if data_bytes.len() < data_end_pos {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "chunk data size exceeds available bytes",
        ));
    }

    let data_bytes = data_bytes[8..data_end_pos].to_vec();
    Ok(interfaces::Chunk::new(chunk_type, data_bytes))
}

pub fn load_file_without_header(file_path: &str) -> Result<Vec<Chunk>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    file.seek(SeekFrom::Start(8))?;
    let mut reader = BufReader::new(file);
    let mut data_bytes = Vec::new();
    reader.read_to_end(&mut data_bytes).err();

    let mut ret_chunks: Vec<Chunk> = Vec::new();

    while data_bytes.len() > 0 {
        let chunk: Chunk = parse_next_chunk(&mut data_bytes)?;
        let _ = data_bytes.drain(0..chunk.data.len());

        println!("{chunk:#?}");
        ret_chunks.push(chunk);
    }

    // let c = interfaces::Chunk::new(chunk_type, data.clone());

    Ok(ret_chunks)
}
