use bytes::Bytes;
use crab_nbt::Nbt;
use gungraun::{prelude::*, Dhat};
use std::hint::black_box;

#[cfg(feature = "serde")]
#[path = "../tests/serde/test_data_definitions.rs"]
mod test_data_definitions;
#[path = "../tests/utils.rs"]
mod utils;

fn read_file(path: &str) -> Bytes {
    utils::read_file(path, true)
}

fn read_file_to_nbt(path: &str) -> Nbt {
    Nbt::read(&mut read_file(path)).expect("Failed to read NBT")
}

#[library_benchmark]
#[bench::complex_player(args = ("tests/data/complex_player.dat"), setup = read_file)]
#[bench::chunk(args = ("tests/data/chunk.nbt"), setup = read_file)]
fn read(value: Bytes) -> Nbt {
    black_box(Nbt::read(&mut black_box(value))).expect("Failed to parse NBT")
}

#[library_benchmark]
#[bench::complex_player(args = ("tests/data/complex_player.dat"), setup = read_file_to_nbt)]
#[bench::chunk(args = ("tests/data/chunk.nbt"), setup = read_file_to_nbt)]
fn write(value: Nbt) -> Bytes {
    black_box(black_box(value).write())
}

library_benchmark_group!(name = read_group, benchmarks = read);
library_benchmark_group!(name = write_group, benchmarks = write);

main!(
    config = LibraryBenchmarkConfig::default().tool(Dhat::default()),
    library_benchmark_groups = [read_group, write_group]
);
