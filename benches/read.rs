use bytes::Bytes;
use crab_nbt::Nbt;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;

#[cfg(feature = "serde")]
#[path = "../tests/serde/test_data_definitions.rs"]
mod test_data_definitions;

fn read_file(file_path: &str) -> Bytes {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    Bytes::from_iter(buffer)
}

fn read_compressed_file(file_path: &str) -> Bytes {
    let mut src = &read_file(file_path)[..];
    let mut decoder = GzDecoder::new(&mut src);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer).unwrap();
    Bytes::from_iter(buffer)
}

fn benchmark_file(criterion: &mut Criterion, file_name: &str, bytes: Bytes) {
    let mut group = criterion.benchmark_group("read");
    group.throughput(Throughput::Bytes(bytes.len() as u64));

    group.bench_function(file_name, |b| {
        b.iter_batched_ref(
            || bytes.clone(),
            |bytes| Nbt::read(bytes).expect("Failed to parse NBT"),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "serde")]
fn benchmark_file_serde<T: serde::de::DeserializeOwned>(
    criterion: &mut Criterion,
    file_name: &str,
    bytes: Bytes,
) {
    let mut group = criterion.benchmark_group("read_serde");
    group.throughput(Throughput::Bytes(bytes.len() as u64));

    group.bench_function(file_name, |b| {
        b.iter_batched(
            || bytes.clone(),
            |mut bytes| {
                crab_nbt::serde::de::from_bytes::<T>(&mut bytes).expect("Failed to parse NBT");
            },
            BatchSize::SmallInput,
        )
    });
}

fn benchmark(criterion: &mut Criterion) {
    let bytes = read_file("tests/data/chunk.nbt");
    benchmark_file(criterion, "chunk", Bytes::clone(&bytes));
    #[cfg(feature = "serde")]
    benchmark_file_serde::<test_data_definitions::Chunk>(criterion, "chunk", bytes);

    let bytes = read_compressed_file("tests/data/complex_player.dat");
    benchmark_file(criterion, "complex_player", Bytes::clone(&bytes));
    #[cfg(feature = "serde")]
    benchmark_file_serde::<test_data_definitions::ComplexPlayer>(
        criterion,
        "complex_player",
        bytes,
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
