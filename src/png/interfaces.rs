use super::helpers;

use super::helpers::u8_to_u32_msb;
use crc32fast::Hasher;
use std::io;
use std::time::UNIX_EPOCH;

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
    pub compressed_bytes: Vec<u8>,
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub struct Png {
    pub chunks: Vec<Chunk>,
    pub ihdr: Ihdr,
    pub idat: Idat,
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

    fn parse_idat_chunk_bytes(data_chunk: &Chunk) -> Option<Vec<u8>> {

        Some(data_chunk.data.clone())
    }

    fn uncompress_idat(idat: Idat) -> Option<Idat> {

        Some(idat)
    }

    pub fn from_file(file_path: &str) -> Result<Png, Box<dyn std::error::Error>> {
        if !helpers::is_valid_png_file_header(file_path) {
            panic!("Not a png file");
        }

        let raw_chunks = helpers::load_file_without_header(file_path)?;
        if &raw_chunks[0].chunk_type_str != "IHDR" {
            panic!("PNG file is missing header");
        }
        let mut ihdr = None;
        let mut idat = Idat {
            compressed_bytes: Vec::new(),
            bytes: Vec::new(),
        };

        for chunk in &raw_chunks {
            match chunk.chunk_type_str.as_str() {
                "IDAT" => {
                    let mut idat_bytes = Png::parse_idat_chunk_bytes(&chunk).unwrap();
                    idat.compressed_bytes.append(&mut idat_bytes);
                }
                "IHDR" => {
                    ihdr = Png::parse_ihdr(&chunk);
                }
                "IEND" => {
                    if chunk.crc != 0xAE426082 {
                        panic!("Invalid IEND crc")
                    }
                    break;
                }
                _ =>  {
                    continue;
                }
            }
        }

        idat = Png::uncompress_idat(idat).unwrap();

        Ok(Png {
            chunks: raw_chunks,
            ihdr: ihdr.unwrap(),
            idat: idat,
        })
    }
}
