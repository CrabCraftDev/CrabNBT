use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
};

use crate::error::Error;
use bytes::{Buf, BufMut};

use simd_cesu8::decode;

// compatibility export
pub use ids::*;

pub mod ids {
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
}

pub const fn get_nbt_type_name(id: u8) -> Option<&'static str> {
    use ids::*;
    Some(match id {
        END_ID => "end",
        BYTE_ID => "byte",
        SHORT_ID => "short",
        INT_ID => "int",
        LONG_ID => "long",
        FLOAT_ID => "float",
        DOUBLE_ID => "double",
        BYTE_ARRAY_ID => "byte array",
        STRING_ID => "string",
        LIST_ID => "list",
        COMPOUND_ID => "compound",
        INT_ARRAY_ID => "int array",
        LONG_ARRAY_ID => "long array",
        _ => return None,
    })
}

pub fn get_nbt_string(bytes: &mut impl Buf) -> Result<String, Error> {
    let len = bytes.try_get_u16()? as usize;
    let string_bytes = bytes.copy_to_bytes(len);
    let string = decode(&string_bytes).map_err(|_| Error::InvalidJavaString)?;
    Ok(string.to_string())
}

pub fn serialize_str_into(s: &str, bytes: &mut impl BufMut) {
    if s.is_empty() {
        bytes.put_u16(0);
        return;
    }

    let java_string = simd_cesu8::encode(s);
    bytes.put_u16(java_string.len() as u16);
    bytes.put_slice(&java_string);
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
        .as_chunks::<N>()
        .0
        .iter()
        .map(|chunk| from_be(*chunk))
        .collect()
}

/// Write an arbitrary list to the [`Formatter`] `f`.
/// Intended for SNBT serialisation.
///
/// The output will look like this:
/// `[{prefix}{elements{affix}, }*]`
/// (Note that the last comma and space will be omitted)
///
/// e.g. `write_listlike(f, "L;", "l", [1,2,3])`
///     will write `[L;1l, 2l, 3l]`
pub(crate) fn write_listlike<T: Display, I: IntoIterator<Item = T>>(
    f: &mut Formatter<'_>,
    prefix: &'static str,
    affix: &'static str,
    arr: I,
) -> fmt::Result {
    write!(f, "[{prefix}")?;
    join_formatted(
        f,
        ", ",
        arr.into_iter()
            .map(|x| move |f: &mut Formatter<'_>| write!(f, "{x}{affix}")),
    )?;
    write!(f, "]")
}

/// like [T]::join, but allowing for formatting
/// Runs a sequence of formatting functions, interspersed with instances of `separator`
pub(crate) fn join_formatted<Separator, I, F>(
    f: &mut Formatter<'_>,
    separator: Separator,
    iterator: I,
) -> fmt::Result
where
    Separator: Clone + Display,
    I: IntoIterator<Item = F>,
    F: FnOnce(&mut Formatter<'_>) -> fmt::Result,
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
    let may_be_unquoted = !s.is_empty()
        && s.chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '+' || c == '-');
    if may_be_unquoted {
        s.to_owned()
    } else {
        escape_string_value(s)
    }
}

pub(crate) fn escape_string_value(s: &str) -> String {
    let mut output = String::with_capacity(s.len() + 2); // +2 because ""
    let mut chosen_quote = None;
    output.push('"'); // placeholder character until we know what quote to use
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

/// Determines the ordering of `a` in regards to `b`.
/// This is basically [Iterator::cmp_by], which as of the time of writing unstable.
/// Once that is stabilised, this function can be replaced.
pub fn compare_by<I1, I2, F>(a: I1, b: I2, mut comparator: F) -> Ordering
where
    I1: IntoIterator,
    I2: IntoIterator,
    F: FnMut(I1::Item, I2::Item) -> Ordering,
{
    let mut i1 = a.into_iter();
    let mut i2 = b.into_iter();
    loop {
        return match (i1.next(), i2.next()) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(a), Some(b)) => {
                let comp = comparator(a, b);
                if comp == Ordering::Equal {
                    continue;
                }
                comp
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_name_no_quotes() {
        assert_eq!(escape_name("hello1234"), "hello1234");
        assert_eq!(escape_name("1234_hello__..WORLD"), "1234_hello__..WORLD");
        assert_eq!(escape_name("...boo"), "...boo");
    }

    #[test]
    fn escape_name_normal_quotes() {
        assert_eq!(escape_name("minecraft:damage"), "\"minecraft:damage\"");
        assert_eq!(escape_name("i haveaspace"), "\"i haveaspace\"");
        assert_eq!(escape_name("i have many spaces"), "\"i have many spaces\"");
        assert_eq!(escape_name("single'double\""), "\"single'double\\\"\"");
    }

    #[test]
    fn escape_name_single_quotes() {
        assert_eq!(
            escape_name("ineed\"special\"handling"),
            "'ineed\"special\"handling'"
        );
        assert_eq!(escape_name("double\"single'"), "'double\"single\\''")
    }
}
