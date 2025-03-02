use crab_nbt::serde::ser::to_bytes_unnamed;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "action", content = "value", rename_all = "snake_case")]
pub enum Event {
    OpenUrl(Cow<'static, str>),
}

#[test]
fn test_enum_unit_variant() {
    let test = Event::OpenUrl("test".to_string().into());
    let bytes = to_bytes_unnamed(&test).unwrap();
    assert_eq!(
        bytes.as_ref(),
        b"\n\x08\0\x06action\0\x08open_url\x08\0\x05value\0\x04test\0"
    );
}
