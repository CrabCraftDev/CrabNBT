use bytes::Bytes;
use crab_nbt::Nbt;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};

#[cfg(feature = "serde")]
#[path = "../tests/serde/test_data_definitions.rs"]
mod test_data_definitions;
#[path = "../tests/utils.rs"]
mod utils;

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
    let bytes = utils::read_file("tests/data/complex_player.dat", true);
    benchmark_file(criterion, "complex_player", Bytes::clone(&bytes));
    #[cfg(feature = "serde")]
    benchmark_file_serde::<test_data_definitions::ComplexPlayer>(
        criterion,
        "complex_player",
        bytes,
    );

    let bytes = utils::read_file("tests/data/chunk.nbt", false);
    benchmark_file(criterion, "chunk", Bytes::clone(&bytes));
    #[cfg(feature = "serde")]
    benchmark_file_serde::<test_data_definitions::Chunk>(criterion, "chunk", bytes);
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
