#[derive(Debug)]
pub struct Chunk {
    // size: u32,
    pub chunk_type: u32,
    data: Vec<u8>,
    crc: u32,
}

// impl fmt::Debug for Chunk {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Chunk")
//         .field("chunk_type", &format_args!("{:#08x}", self.chunk_type))
//         .field("data", &format_args!("{:#08x}", self.chunk_type))
//         .field("crc", &format_args!("{:#08x}", self.chunk_type))
//         .finish()
//     }
// }

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
    magic_bytes: [u8; 8],
    chunks: Vec<Chunk>,
}
