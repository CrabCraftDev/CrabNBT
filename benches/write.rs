use crab_nbt::Nbt;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};

#[cfg(feature = "serde")]
#[path = "../tests/serde/test_data_definitions.rs"]
mod test_data_definitions;
#[path = "../tests/utils.rs"]
mod utils;

fn benchmark_file(criterion: &mut Criterion, file_name: &str, nbt: &Nbt, bytes_len: usize) {
    let mut group = criterion.benchmark_group("write");
    group.throughput(Throughput::Bytes(bytes_len as u64));

    group.bench_function(file_name, |b| {
        b.iter_batched_ref(|| nbt, |nbt| nbt.write(), BatchSize::SmallInput)
    });
}

#[cfg(feature = "serde")]
fn benchmark_file_serde<T>(criterion: &mut Criterion, file_name: &str, bytes: &[u8])
where
    T: serde::de::DeserializeOwned + serde::ser::Serialize,
{
    let mut group = criterion.benchmark_group("write_serde");
    group.throughput(Throughput::Bytes(bytes.len() as u64));

    let deserialized_struct =
        crab_nbt::serde::de::from_bytes::<T>(&bytes).expect("Failed to parse NBT");

    group.bench_function(file_name, |b| {
        b.iter_batched_ref(
            || &deserialized_struct,
            |deserialized_struct| {
                crab_nbt::serde::ser::to_bytes::<T>(deserialized_struct, "".to_owned())
                    .expect("Failed to serialize NBT");
            },
            BatchSize::SmallInput,
        )
    });
}

fn benchmark(criterion: &mut Criterion) {
    let bytes = utils::read_file("tests/data/complex_player.dat", true);
    let nbt = Nbt::read(&bytes).expect("Failed to parse NBT");
    benchmark_file(criterion, "complex_player", &nbt, bytes.len());
    #[cfg(feature = "serde")]
    benchmark_file_serde::<test_data_definitions::ComplexPlayer>(
        criterion,
        "complex_player",
        &bytes,
    );

    let bytes = utils::read_file("tests/data/chunk.nbt", false);
    let nbt = Nbt::read(&bytes).expect("Failed to parse NBT");
    benchmark_file(criterion, "chunk", &nbt, bytes.len());
    #[cfg(feature = "serde")]
    benchmark_file_serde::<test_data_definitions::Chunk>(criterion, "chunk", &bytes);
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
