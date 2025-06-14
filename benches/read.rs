use bytes::Bytes;
use crab_nbt::Nbt;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;

#[cfg(feature = "serde")]
#[path = "../tests/serde/test_data_definitions.rs"]
mod test_data_definitions;

fn decompress_data(file_path: &str) -> Vec<u8> {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    let mut src = &buffer[..];

    let mut src_decoder = GzDecoder::new(&mut src);
    let mut input = Vec::new();
    if src_decoder.read_to_end(&mut input).is_err() {
        input = buffer;
    }

    input
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = decompress_data("tests/data/complex_player.dat");

    let bytes = Bytes::from_iter(input);

    let mut group = c.benchmark_group("read");
    group.throughput(Throughput::Bytes(bytes.len() as u64));

    group.bench_function("read_complex_player_nbt", |b| {
        b.iter_batched_ref(
            || bytes.clone(),
            |bytes| Nbt::read(bytes).expect("Failed to parse NBT"),
            BatchSize::SmallInput,
        )
    });

    #[cfg(feature = "serde")]
    group.bench_function("read_complex_player_nbt_serde", |b| {
        b.iter_batched_ref(
            || bytes.clone(),
            |bytes| {
                crab_nbt::serde::de::from_bytes::<test_data_definitions::ComplexPlayer>(bytes)
                    .expect("Failed to parse NBT")
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
