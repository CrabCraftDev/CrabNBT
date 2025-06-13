use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Serde struct definitions for files in `tests/data/`
// Used in tests and benchmarks

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BigTest {
    #[serde(rename = "nested compound test")]
    nested_compound_test: NestedCompoundTest,

    #[serde(rename = "intTest")]
    int_test: i32,

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

// Example implementation of Chunk and related structures
// Note: not all fields are implemented, just a subset for demonstration purposes
#[derive(Debug, Serialize, Deserialize)]
pub struct Chunk {
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    #[serde(rename = "xPos")]
    pub x_pos: i32,
    #[serde(rename = "zPos")]
    pub z_pos: i32,
    #[serde(rename = "yPos")]
    pub y_pos: i32,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "LastUpdate")]
    pub last_update: i64,
    pub sections: Vec<RawSection>,
    #[serde(rename = "block_entities")]
    pub block_entities: Option<Vec<BlockEntity>>,
    #[serde(rename = "Heightmaps")]
    pub heightmaps: Option<Heightmaps>,
    #[serde(rename = "Lights")]
    pub lights: Option<Vec<Vec<i16>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawSection {
    #[serde(rename = "Y")]
    pub y: i8,
    pub block_states: BlockStates,
    pub biomes: Biomes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockEntity {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    #[serde(rename = "Y")]
    pub y: i8,
    #[serde(rename = "block_states")]
    pub block_states: BlockStates,
    pub biomes: Biomes,
    #[serde(rename = "BlockLight")]
    pub block_light: Option<Vec<u8>>,
    #[serde(rename = "SkyLight")]
    pub sky_light: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockStates {
    pub palette: Vec<BlockStatePaletteEntry>,
    pub data: Option<Vec<i64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockStatePaletteEntry {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Properties")]
    pub properties: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomesPaletteEntry {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Properties")]
    pub properties: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Biomes {
    pub palette: Vec<String>,
    pub data: Option<Vec<i64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Heightmaps {
    #[serde(rename = "MOTION_BLOCKING")]
    pub motion_blocking: Vec<i64>,
    #[serde(rename = "WORLD_SURFACE")]
    pub world_surface: Vec<i64>,
}
