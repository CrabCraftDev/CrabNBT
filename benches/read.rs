use crab_nbt::Nbt;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use std::fs;

#[cfg(feature = "serde")]
#[path = "../tests/serde/test_data_definitions.rs"]
mod test_data_definitions;

#[path = "../tests/decompress.rs"]
mod decompress;

fn criterion_benchmark(c: &mut Criterion) {
    let input =
        decompress::decompress_data(&fs::read("tests/data/complex_player.dat").unwrap()[..]);

    let mut group = c.benchmark_group("read");
    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("read_complex_player_nbt", |b| {
        b.iter_batched_ref(
            || &input[..],
            |bytes| Nbt::read(bytes).expect("Failed to parse NBT"),
            BatchSize::SmallInput,
        )
    });

    #[cfg(feature = "serde")]
    group.bench_function("read_complex_player_nbt_serde", |b| {
        b.iter_batched_ref(
            || &input[..],
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
