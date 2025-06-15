use std::io::Read;

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
