use bytes::{Bytes, BytesMut};
use crab_nbt::Nbt;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;

fn criterion_benchmark(c: &mut Criterion) {
    let mut file = File::open("tests/data/complex_player.dat").expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    let mut src = &buffer[..];

    let mut src_decoder = GzDecoder::new(&mut src);
    let mut input = Vec::new();
    if src_decoder.read_to_end(&mut input).is_err() {
        input = buffer;
    }

    let bytes = Bytes::from_iter(input);
    let bytes_mut = BytesMut::from(bytes);

    let mut group = c.benchmark_group("read");
    group.throughput(Throughput::Bytes(bytes_mut.len() as u64));

    group.bench_function("read_bigtest_nbt", |b| {
        b.iter(|| {
            let output = Nbt::read(&mut bytes_mut.clone()).expect("Failed to parse NBT");
            black_box(output)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
