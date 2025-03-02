use crab_nbt::serde::de::from_bytes_unnamed;
use crab_nbt::serde::ser::to_bytes_unnamed;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum Message {
    Request { id: String, method: i8 },
    Response { uid: i8 },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Text {
    #[serde(flatten)]
    pub message: Message,
}

#[test]
fn test_map_cycle() {
    let test = Text {
        message: Message::Request {
            id: "a".to_string(),
            method: 0,
        },
    };
    let mut bytes = to_bytes_unnamed(&test).unwrap();
    let result: Text = from_bytes_unnamed(&mut bytes).unwrap();
    assert_eq!(result, test);
}
