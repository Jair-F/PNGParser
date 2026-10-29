use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use super::interfaces;

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
