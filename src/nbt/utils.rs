use std::fmt::{Display, Formatter};

use crate::error::Error;
use bytes::Buf;
use simd_cesu8::decode;

pub const END_ID: u8 = 0;
pub const BYTE_ID: u8 = 1;
pub const SHORT_ID: u8 = 2;
pub const INT_ID: u8 = 3;
pub const LONG_ID: u8 = 4;
pub const FLOAT_ID: u8 = 5;
pub const DOUBLE_ID: u8 = 6;
pub const BYTE_ARRAY_ID: u8 = 7;
pub const STRING_ID: u8 = 8;
pub const LIST_ID: u8 = 9;
pub const COMPOUND_ID: u8 = 10;
pub const INT_ARRAY_ID: u8 = 11;
pub const LONG_ARRAY_ID: u8 = 12;

pub fn get_nbt_string(bytes: &mut impl Buf) -> Result<String, Error> {
    let len = bytes.get_u16() as usize;
    let string_bytes = bytes.copy_to_bytes(len);
    let string = decode(&string_bytes).map_err(|_| Error::InvalidJavaString)?;
    Ok(string.to_string())
}

// This can be improved once rust-lang/rust#132980 is resolved:
// Instead of passing `BYTES` manually, we could use const generics, e.g. `size_of::<T>()`.
pub(crate) fn read_array<T, const N: usize, F>(
    bytes: &mut impl Buf,
    len: usize,
    from_be: F,
) -> Vec<T>
where
    F: Fn([u8; N]) -> T,
{
    bytes
        .copy_to_bytes(len * N)
        .chunks_exact(N)
        .map(|chunk| {
            let arr: [u8; N] = chunk.try_into().expect("chunk size mismatch");
            from_be(arr)
        })
        .collect()
}

// like [T]::join, but allowing for formatting
// Runs a sequence of formatting functions, interspersed with instances of `separator`
pub(crate) fn join_formatted<Separator, I, F>
    (f: &mut Formatter<'_>, separator: Separator, iterator: I) -> std::fmt::Result 
    where Separator: Clone + Display, 
            I: IntoIterator<Item = F>, 
            F: FnOnce(&mut Formatter<'_>) -> std::fmt::Result,
{
    let mut peekable = iterator.into_iter().peekable();
    while let Some(function) = peekable.next() {
        function(f)?;
        if peekable.peek().is_some() {
            write!(f, "{}", separator)?;
        }
    }
    Ok(())
}

pub(crate) fn escape_name(s: &str) -> String {
    let may_be_unquoted = !s.is_empty() && s.chars()
        .all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '+' || c == '-' );
    if may_be_unquoted { s.to_owned() } 
    else { escape_string_value(s) }
}

pub(crate) fn escape_string_value(s: &str) -> String {
    let mut output = String::with_capacity(s.len() + 2); // +2 because ""
    let mut chosen_quote = None;
    output.push('"');  // placeholder character until we know what quote to use
    for c in s.chars() {
        if c == '\\' {
            output.push('\\');
        } else if c == '"' || c == '\'' {
            if chosen_quote.is_none() {
                chosen_quote = Some(if c == '"' { '\'' } else { '"' });
            }
            if chosen_quote.map(|q| q == c).unwrap_or(false) {
                output.push('\\');
            }
        }
        output.push(c);
    }

    let escape_char = chosen_quote.unwrap_or('\"');
    output.replace_range(0..1, &escape_char.to_string());
    output.push(escape_char);
    output
}
