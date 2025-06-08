use bytes::Bytes;
use crab_nbt::Nbt;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Read;

#[cfg(feature = "serde")]
mod serde {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct ComplexPlayer {
        #[serde(rename = "SelectedItemSlot")]
        selected_item_slot: i32,
        #[serde(rename = "UUIDLeast")]
        uuid_least: i64,
        #[serde(rename = "Attributes")]
        attributes: Vec<Attribute>,
        #[serde(rename = "Inventory")]
        inventory: Vec<Item>,
        #[serde(rename = "Health")]
        health: i16,
        #[serde(rename = "XpLevel")]
        xp_level: i32,
        #[serde(rename = "SelectedItem")]
        selected_item: Item,
        #[serde(rename = "ActiveEffects")]
        active_effects: Vec<Effect>,
        #[serde(rename = "Pos")]
        pos: Vec<f64>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Attribute {
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "Base")]
        base: f64,
        #[serde(rename = "Modifiers", default)]
        modifiers: Vec<Modifier>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Modifier {
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "UUIDLeast")]
        uuid_least: i64,
        #[serde(rename = "UUIDMost")]
        uuid_most: i64,
        #[serde(rename = "Operation")]
        operation: i32,
        #[serde(rename = "Amount")]
        amount: f64,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Item {
        id: String,
        #[serde(rename = "Damage")]
        damage: i16,
        #[serde(rename = "Count")]
        count: u8,
        #[serde(rename = "Slot", default)]
        slot: Option<u8>,
        #[serde(rename = "tag")]
        tag: Option<ItemTag>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct ItemTag {
        #[serde(rename = "RepairCost")]
        repair_cost: Option<i32>,
        #[serde(rename = "ench")]
        ench: Vec<Enchantment>,
        #[serde(rename = "display")]
        display: Option<Display>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Enchantment {
        #[serde(rename = "id")]
        id: i16,
        #[serde(rename = "lvl")]
        lvl: i16,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Display {
        #[serde(rename = "Name")]
        name: String,
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Effect {
        #[serde(rename = "Id")]
        id: u8,
        #[serde(rename = "Amplifier")]
        amplifier: u8,
        #[serde(rename = "Duration")]
        duration: i32,
        #[serde(rename = "Ambient")]
        ambient: u8,
        #[serde(rename = "ShowParticles")]
        show_particles: u8,
    }
}

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
                crab_nbt::serde::de::from_bytes::<serde::ComplexPlayer>(bytes)
                    .expect("Failed to parse NBT")
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
