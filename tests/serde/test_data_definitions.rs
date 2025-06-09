use crab_nbt::serde::arrays::BytesArray;
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
        rename = "byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))",
        with = "BytesArray"
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
    pub selected_item_slot: i32,
    #[serde(rename = "UUIDLeast")]
    pub uuid_least: i64,
    #[serde(rename = "Attributes")]
    pub attributes: Vec<Attribute>,
    #[serde(rename = "Motion")]
    pub motion: Vec<f64>,
    #[serde(rename = "foodExhaustionLevel")]
    pub food_exhaustion_level: f32,
    #[serde(rename = "foodTickTimer")]
    pub food_tick_timer: i32,
    #[serde(rename = "XpLevel")]
    pub xp_level: i32,
    #[serde(rename = "Health")]
    pub health: i16,
    #[serde(rename = "XpSeed")]
    pub xp_seed: i32,
    #[serde(rename = "HealF")]
    pub heal_f: f32,
    #[serde(rename = "SelectedItem")]
    pub selected_item: Item,
    #[serde(rename = "SpawnForced")]
    pub spawn_forced: i8,
    #[serde(rename = "Inventory")]
    pub inventory: Vec<Item>,
    #[serde(rename = "ActiveEffects")]
    pub active_effects: Vec<Effect>,
    #[serde(rename = "Sleeping")]
    pub sleeping: i8,
    #[serde(rename = "SpawnX")]
    pub spawn_x: i32,
    #[serde(rename = "SpawnY")]
    pub spawn_y: i32,
    #[serde(rename = "Fire")]
    pub fire: i16,
    #[serde(rename = "SpawnZ")]
    pub spawn_z: i32,
    #[serde(rename = "playerGameType")]
    pub player_game_type: i32,
    #[serde(rename = "foodLevel")]
    pub food_level: i32,
    #[serde(rename = "Score")]
    pub score: i32,
    #[serde(rename = "Invulnerable")]
    pub invulnerable: i8,
    #[serde(rename = "DeathTime")]
    pub death_time: i16,
    #[serde(rename = "EnderItems")]
    pub ender_items: Vec<Item>,
    #[serde(rename = "XpP")]
    pub xp_p: f32,
    #[serde(rename = "AbsorptionAmount")]
    pub absorption_amount: f32,
    #[serde(rename = "SleepTimer")]
    pub sleep_timer: i16,
    #[serde(rename = "OnGround")]
    pub on_ground: i8,
    #[serde(rename = "HurtTime")]
    pub hurt_time: i16,
    #[serde(rename = "UUIDMost")]
    pub uuid_most: i64,
    #[serde(rename = "HurtByTimestamp")]
    pub hurt_by_timestamp: i32,
    #[serde(rename = "Dimension")]
    pub dimension: i32,
    #[serde(rename = "Air")]
    pub air: i16,
    #[serde(rename = "Pos")]
    pub pos: Vec<f64>,
    #[serde(rename = "foodSaturationLevel")]
    pub food_saturation_level: f32,
    #[serde(rename = "PortalCooldown")]
    pub portal_cooldown: i32,
    #[serde(rename = "abilities")]
    pub abilities: Abilities,
    #[serde(rename = "FallDistance")]
    pub fall_distance: f32,
    #[serde(rename = "XpTotal")]
    pub xp_total: i32,
    #[serde(rename = "Rotation")]
    pub rotation: Vec<f32>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Abilities {
    pub flying: i8,
    pub instabuild: i8,
    pub mayfly: i8,
    pub invulnerable: i8,
    #[serde(rename = "mayBuild")]
    pub may_build: i8,
    #[serde(rename = "flySpeed")]
    pub fly_speed: f32,
    #[serde(rename = "walkSpeed")]
    pub walk_speed: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Attribute {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Base")]
    base: f64,
    #[serde(rename = "Modifiers", default, skip_serializing_if = "Vec::is_empty")]
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
pub struct Item {
    id: String,
    #[serde(rename = "Damage")]
    damage: i16,
    #[serde(rename = "Count")]
    count: i8,
    #[serde(rename = "Slot", default)]
    slot: Option<i8>,
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
pub struct Effect {
    #[serde(rename = "Id")]
    id: i8,
    #[serde(rename = "Amplifier")]
    amplifier: i8,
    #[serde(rename = "Duration")]
    duration: i32,
    #[serde(rename = "Ambient")]
    ambient: i8,
    #[serde(rename = "ShowParticles")]
    show_particles: i8,
}
