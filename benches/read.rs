use std::fs;

use bytes::Bytes;
use crab_nbt::Nbt;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};

#[cfg(feature = "serde")]
#[path = "../tests/serde/test_data_definitions.rs"]
mod test_data_definitions;

#[path = "../tests/utils.rs"]
mod utils;

fn criterion_benchmark(c: &mut Criterion) {
    let input = utils::decompress_data(&fs::read("tests/data/complex_player.dat").unwrap()[..]);

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
