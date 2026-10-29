use super::constants;
use std::fs::File;
use super::interfaces;
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
    let mut file_sig: u64 = (u8_to_u32_msb(&png_bytes[0..4].try_into().unwrap()) as u64) << 32;
    file_sig += u8_to_u32_msb(&png_bytes[4..8].try_into().unwrap()) as u64;

    file_sig == constants::PNG_FILE_MAGIC_HEADER
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



// fn read_next_chunk(data_bytes: &mut Vec<u8>) -> io::Result<interfaces::Chunk> {
//     let chunk_size: u32 = data_bytes[0..5].conca

//     let mut ret = interfaces::Chunk::new(chunk_type, data);

//     unimplemented!("Not implemented")
// }

pub fn read_png(file_path: &str) -> Result<interfaces::Chunk, Box<dyn std::error::Error>> {
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
