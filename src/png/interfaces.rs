use super::helpers;

use std::io;
use crc32fast::Hasher;

#[derive(Debug)]
pub struct Chunk {
    pub chunk_type: u32,
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
    pub fn new(chunk_type: u32, data: Vec<u8>, chunk_crc:u32) -> io::Result<Chunk> {
        let calculated_crc = calc_crc(chunk_type, &data);
        if calculated_crc != chunk_crc {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("CRC mismatch: expected {:#010X}, found {:#010X}", chunk_crc, calculated_crc),
            ));

        }
        Ok(Chunk {
            chunk_type: chunk_type,
            data: data,
            crc: calculated_crc,
        })
    }
}

#[derive(Debug)]
pub struct PNG {
    pub chunks: Vec<Chunk>,
}

impl PNG {
    pub fn from_file(file_path: &str) -> Result<PNG, Box<dyn std::error::Error>> {
        if !helpers::is_valid_png_file_header(file_path) {
            panic!("Not a png file");
        }

        // unimplemented!("Not implemented yet");
        let ret = PNG {
            chunks: helpers::load_file_without_header(file_path)?
        };
        Ok(ret)
    }
}
