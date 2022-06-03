#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate ruzstd;
use std::io::Read;

fn decode_ruzstd(data: &mut dyn std::io::Read) -> Vec<u8> {
    let mut decoder = ruzstd::StreamingDecoder::new(data).unwrap();
    let mut result: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut result).expect("Decoding failed");
    result
}

fn decode_ruzstd_writer(mut data: impl Read) -> Vec<u8> {
    let mut decoder = ruzstd::FrameDecoder::new();
    decoder.reset(&mut data).unwrap();
    let mut result = vec![];
    while !decoder.is_finished() || decoder.can_collect() > 0 {
        decoder
            .decode_blocks(
                &mut data,
                ruzstd::BlockDecodingStrategy::UptoBytes(1024 * 1024),
            )
            .unwrap();
        decoder.collect_to_writer(&mut result).unwrap();
    }
    result
}

fn encode_zstd(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    zstd::stream::encode_all(std::io::Cursor::new(data), 3)
}

fuzz_target!(|data: &[u8]| {
    let compressed = encode_zstd(data).unwrap();
    let decoded = decode_ruzstd(&mut compressed.as_slice());
    let decoded2 = decode_ruzstd_writer(&mut compressed.as_slice());
    assert!(
        decoded == data,
        "Decoded data did not match the original input"
    );
    assert_eq!(
        decoded2, data,
        "Decoded data did not match the original input"
    );
});
