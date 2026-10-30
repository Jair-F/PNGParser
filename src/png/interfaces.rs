use super::helpers;

use crc32fast::Hasher;
use std::io;
use super::helpers::u8_to_u32_msb;

#[derive(Debug)]
pub struct Chunk {
    pub chunk_type: u32,
    pub chunk_type_str: String,
    pub data: Vec<u8>,
    pub crc: u32,
}

pub fn calc_crc(chunk_type: u32, chunk_data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();

    let chunk_types: [u8; 4] = chunk_type.to_be_bytes();
    // 1. Hash the 4-byte chunk type ASCII identifier
    hasher.update(&chunk_types);

    // 2. Hash the variable-length payload data
    hasher.update(chunk_data);

    // 3. Finalize and compare
    hasher.finalize()
}

impl Chunk {
    pub fn new(chunk_type: u32, data: Vec<u8>, chunk_crc: u32) -> io::Result<Chunk> {
        let calculated_crc = calc_crc(chunk_type, &data);
        if calculated_crc != chunk_crc {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "CRC mismatch: expected {:#010X}, found {:#010X}",
                    chunk_crc, calculated_crc
                ),
            ));
        }

        let chunk_type_name: String = chunk_type
            .to_be_bytes()
            .iter()
            .map(|&b| b as char)
            .collect();

        Ok(Chunk {
            chunk_type: chunk_type,
            chunk_type_str: chunk_type_name,
            data: data,
            crc: calculated_crc,
        })
    }
}

pub struct Ihdr {
    pub width:u32,
    pub height:u32,
    pub bit_depth:u8,
    pub color_type:u8,
    pub compression:u8,
    pub filter:u8,
    pub interlace:u8,
}

#[derive(Debug)]
pub struct Png {
    pub chunks: Vec<Chunk>
}

impl Png {
    pub fn from_file(file_path: &str) -> Result<Png, Box<dyn std::error::Error>> {
        if !helpers::is_valid_png_file_header(file_path) {
            panic!("Not a png file");
        }

        let raw_chunks = helpers::load_file_without_header(file_path)?;
        let first_chunk = &raw_chunks[0];
        if first_chunk.chunk_type_str != "IHDR" {
            panic!("PNG file is missing header");
        }
        
        Png::parse_ihdr(first_chunk);

        // unimplemented!("Not implemented yet");
        let ret = Png {
            chunks: raw_chunks,
        };
        Ok(ret)
    }

    fn parse_ihdr(header_chunk: &Chunk) {
        let data = &header_chunk.data;
        let width = u8_to_u32_msb(data[0..4].try_into().unwrap());
        let height = u8_to_u32_msb(data[4..8].try_into().unwrap());
        let bit_depth = data[8];
        let color_type = data[9];
        let compression = data[10];
        let filter = data[11];
        let interlace = data[12];
    }
}
