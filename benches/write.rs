use crab_nbt::Nbt;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use std::fs::{self};

#[cfg(feature = "serde")]
#[path = "../tests/serde/test_data_definitions.rs"]
mod test_data_definitions;

#[path = "../tests/decompress.rs"]
mod decompress;

fn criterion_benchmark(c: &mut Criterion) {
    let input =
        decompress::decompress_data(&fs::read("tests/data/complex_player.dat").unwrap()[..]);

    let nbt = Nbt::read(&input[..]).expect("Failed to parse NBT");

    let mut group = c.benchmark_group("write");

    group.throughput(Throughput::Bytes(input.len() as u64));

    group.bench_function("write_complex_player_nbt", |b| {
        b.iter_batched_ref(|| &nbt, |nbt| nbt.write(), BatchSize::SmallInput)
    });

    #[cfg(feature = "serde")]
    {
        let complex_player =
            crab_nbt::serde::de::from_bytes::<test_data_definitions::ComplexPlayer>(&input[..])
                .expect("Failed to parse NBT");

        group.bench_function("write_complex_player_nbt_serde", |b| {
            b.iter_batched_ref(
                || &complex_player,
                |complex_player| {
                    crab_nbt::serde::ser::to_bytes::<test_data_definitions::ComplexPlayer>(
                        complex_player,
                        "".to_owned(),
                    )
                    .expect("Failed to serialize NBT");
                },
                BatchSize::SmallInput,
            )
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
