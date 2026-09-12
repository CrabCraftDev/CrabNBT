use crab_nbt::Nbt;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

#[path = "../tests/utils.rs"]
mod utils;

fn benchmark_ser(criterion: &mut Criterion, name: &str, nbt: &Nbt) {
    let mut group = criterion.benchmark_group("snbt-ser");

    group.bench_function(name, |bencher| {
        bencher.iter_batched(|| nbt, |data| data.to_string(), BatchSize::SmallInput);
    });
}

fn benchmark(criterion: &mut Criterion) {
    let bytes = utils::read_file("tests/data/complex_player.dat", true);
    let nbt = Nbt::read(&mut bytes.clone()).expect("Failed to parse NBT");
    benchmark_ser(criterion, "complex_player", &nbt);

    let bytes = utils::read_file("tests/data/chunk.nbt", false);
    let nbt = Nbt::read(&mut bytes.clone()).expect("Failed to parse NBT");
    benchmark_ser(criterion, "chunk", &nbt);
}

criterion_group!(benches, benchmark);

criterion_main!(benches);