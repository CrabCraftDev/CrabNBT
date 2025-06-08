use crab_nbt::serde::de::from_bytes_unnamed;
use crab_nbt::serde::ser::to_bytes_unnamed;
use crab_nbt::Nbt;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Text {
    #[serde(flatten)]
    pub content: TextContent,
    #[serde(flatten)]
    pub style: TextStyle,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "extra")]
    pub children: Vec<Text>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct TextStyle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<i8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
#[repr(u8)]
pub enum TextContent {
    Literal { text: Cow<'static, str> },
}

#[test]
pub fn test_complex_text() {
    let data = Text {
        content: TextContent::Literal { text: "abc".into() },
        style: TextStyle { bold: None },
        children: vec![Text {
            style: TextStyle { bold: Some(1) },
            content: TextContent::Literal {
                text: "inner".into(),
            },
            children: vec![],
        }],
    };

    let mut text = to_bytes_unnamed(&data).unwrap();
    let serialized: Text = from_bytes_unnamed(&mut text).unwrap();
    assert_eq!(serialized, data);

    let mut text = to_bytes_unnamed(&data).unwrap();
    assert!(Nbt::read_unnamed(&mut text).is_ok())
}
