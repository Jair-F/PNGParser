use crate::png::constants::{
    COLOR_TYPE_ALPHA_GRAYSCALE, COLOR_TYPE_ALPHA_TRUECOLOR, COLOR_TYPE_GRAYSCALE,
    COLOR_TYPE_INDEXED, COLOR_TYPE_TRUECOLOR_RGB,
};

use super::helpers;

use super::helpers::u8_to_u32_msb;
use crc32fast::Hasher;
use flate2::read::ZlibDecoder;
use std::io;
use std::io::{Read, Result};

#[derive(Debug)]
pub struct Chunk {
    pub chunk_type_str: String,
    pub data: Vec<u8>,
    pub crc: u32,
}

#[derive(Debug)]
pub struct Ihdr {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression: u8,
    pub filter: u8,
    pub interlace: u8,
}

#[derive(Debug)]
pub struct Idat {
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub struct Png {
    pub chunks: Vec<Chunk>,
    pub ihdr: Ihdr,
    pub idat: Idat,
}

impl Ihdr {
    fn num_of_color_channels(&self) -> u8 {
        match self.color_type {
            COLOR_TYPE_GRAYSCALE => 1,
            COLOR_TYPE_TRUECOLOR_RGB => 3,
            COLOR_TYPE_INDEXED => 1,
            COLOR_TYPE_ALPHA_GRAYSCALE => 2,
            COLOR_TYPE_ALPHA_TRUECOLOR => 4,
            _ => panic!("Unknown Color type in IHDR"),
        }
    }

    pub fn bytes_per_pixel(&self) -> usize {
        let channels = self.num_of_color_channels() as usize;
        let bit_depth = self.bit_depth as usize;

        match bit_depth {
            1 | 2 | 4 | 8 | 16 => {}
            _ => panic!("Unknown bitdepth in IHDR"),
        }
        let bits_per_pixel = bit_depth * channels;

        if bits_per_pixel < 8 {
            1
        } else {
            bits_per_pixel / 8
        }
    }
}

impl Chunk {
    pub fn new(chunk_type: u32, data: Vec<u8>, chunk_crc: u32) -> io::Result<Chunk> {
        let calculated_crc = Chunk::calc_crc(chunk_type, &data);
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
            chunk_type_str: chunk_type_name,
            data: data,
            crc: calculated_crc,
        })
    }

    pub fn calc_crc(chunk_type: u32, chunk_data: &[u8]) -> u32 {
        let mut hasher = Hasher::new();
        let chunk_types: [u8; 4] = chunk_type.to_be_bytes();

        hasher.update(&chunk_types);
        hasher.update(chunk_data);
        hasher.finalize()
    }
}

impl Png {
    fn parse_ihdr(header_chunk: &Chunk) -> Option<Ihdr> {
        let data = &header_chunk.data;

        Some(Ihdr {
            width: u8_to_u32_msb(data[0..4].try_into().unwrap()),
            height: u8_to_u32_msb(data[4..8].try_into().unwrap()),
            bit_depth: data[8],
            color_type: data[9],
            compression: data[10],
            filter: data[11],
            interlace: data[12],
        })
    }

    fn uncompress_idat(ihdr: &Ihdr, compressed_bytes: &[u8]) -> Result<Idat> {
        if ihdr.compression != 0 {
            panic!("Unsupported compression algorithm");
        }

        let mut decoder = ZlibDecoder::new(compressed_bytes);
        let mut decompressed = Vec::new();

        decoder.read_to_end(&mut decompressed)?;

        Ok(Idat {
            bytes: decompressed,
        })
    }

    pub fn from_file(file_path: &str) -> Result<Png> {
        if !helpers::is_valid_png_file_header(file_path) {
            panic!("Not a png file");
        }

        let raw_chunks = helpers::load_file_without_header(file_path).unwrap();
        if &raw_chunks[0].chunk_type_str != "IHDR" {
            panic!("PNG file is missing header");
        }
        let mut ihdr = None;
        let mut compressed_idat_bytes = Vec::new();

        for chunk in &raw_chunks {
            match chunk.chunk_type_str.as_str() {
                "IDAT" => {
                    compressed_idat_bytes.extend_from_slice(&chunk.data);
                }
                "IHDR" => {
                    ihdr = Png::parse_ihdr(&chunk);
                }
                "IEND" => {
                    break;
                }
                _ => {
                    continue;
                }
            }
        }

        let ihdr = ihdr.unwrap();
        let idat = Png::uncompress_idat(&ihdr, &compressed_idat_bytes[..]).unwrap();

        Ok(Png {
            chunks: raw_chunks,
            ihdr: ihdr,
            idat: idat,
        })
    }
}
