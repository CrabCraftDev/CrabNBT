use std::{fs, io::Read};

use bytes::Bytes;
use flate2::bufread::GzDecoder;

#[allow(unused)]
pub fn decompress_data(buffer: &[u8]) -> Vec<u8> {
    let mut decoder = GzDecoder::new(buffer);
    let mut input = Vec::new();
    if decoder.read_to_end(&mut input).is_err() {
        input = buffer.to_vec();
    }

    input
}

#[allow(unused)]
pub fn read_file(file_path: &str, compressed: bool) -> Bytes {
    let data = &fs::read(file_path).expect("Failed to open file")[..];
    if compressed {
        Bytes::from_iter(decompress_data(data))
    } else {
        Bytes::from_iter(data.iter().copied())
    }
}
