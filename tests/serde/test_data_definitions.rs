use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Serde struct definitions for files in `tests/data/`
// Used in tests and benchmarks

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BigTest {
    #[serde(rename = "nested compound test")]
    nested_compound_test: NestedCompoundTest,

    #[serde(rename = "byteTest")]
    byte_test: i8,

    #[serde(rename = "stringTest")]
    string_test: String,

    #[serde(rename = "listTest (long)")]
    list_test_long: Vec<i64>,

    #[serde(rename = "doubleTest")]
    double_test: f64,

    #[serde(rename = "floatTest")]
    float_test: f32,

    #[serde(rename = "longTest")]
    long_test: i64,

    #[serde(rename = "listTest (compound)")]
    list_test_compound: Vec<CompoundEntry>,

    #[serde(
        rename = "byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))"
    )]
    byte_array_test: Vec<i8>,

    #[serde(rename = "shortTest")]
    short_test: i16,

    #[serde(flatten)]
    pub extra: HashMap<String, crab_nbt::NbtTag>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct NestedCompoundTest {
    egg: NamedValue,
    ham: NamedValue,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct NamedValue {
    name: String,
    value: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct CompoundEntry {
    #[serde(rename = "created-on")]
    created_on: i64,
    name: String,
}

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
