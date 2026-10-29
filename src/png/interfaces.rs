use super::helpers;
use std::io;

#[derive(Debug)]
pub struct Chunk {
    pub chunk_type: u32,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: u32, data: Vec<u8>) -> Chunk {
        let crc = crc32fast::hash(&data);
        Chunk {
            chunk_type: chunk_type,
            data: data,
            crc: crc,
        }
    }
}

#[derive(Debug)]
pub struct PNG {
    // magic_bytes: [u8; 8],
    chunks: Vec<Chunk>,
}

impl PNG {
    pub fn from_file(file_path: &str) -> io::Result<PNG> {
        if !helpers::is_valid_png_file_header(file_path) {
            panic!("Not a png file");
        }

        unimplemented!("Not implemented yet");
    }
}
