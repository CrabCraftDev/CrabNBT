/// Macro that simplifies the creation of Nbt using JSON/SNBT-like syntax.
/// It takes a name and a content block, and returns an `Nbt` object.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use crab_nbt::nbt;
///
/// let nbt = nbt!("root", {
///     "float": 1.0,
///     "key": "value",
///     "long_array": [L; 1],
///     "int_array": [Int; 1],
///     "list": ["a", "b", "c"],
///     "nbt_inner": {
///         "key": "sub value"
///     }
/// });
/// ```
///
/// This will create an `Nbt` object with the name "root" and the specified content.
///
/// # Parameters
///
/// - `name`: The name of the NBT. This should be a string literal.
/// - `content`: The content of the NBT. This should be a block containing key-value pairs, where the keys are string literals and the values are valid NBT data types.
///
/// # Returns
///
/// An `Nbt` object with the specified name and content.
#[cfg(feature = "macro")]
#[macro_export]
macro_rules! nbt {
    ($name:expr, $content:tt) => {
        $crate::Nbt::new($name, $crate::nbt_inner!($content))
    };
}

#[cfg(feature = "macro")]
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! nbt_inner {
    ({ $($key:tt : $value:tt),* $(,)? }) => {
        $crate::NbtCompound::new({
            #[allow(unused_mut)]
            let mut map = ::std::collections::HashMap::<::std::string::String, $crate::NbtTag>::new();
            $(map.insert($key.into(), nbt_inner!(@value $value));)*
            map
        })
    };
    (@value $ident:ident) => { $crate::NbtTag::from($ident) };
    (@value $lit:literal) => { $crate::NbtTag::from($lit) };
    (@value $other:tt) => { $crate::NbtTag::from(nbt_inner!($other)) };
    ([L; $($lit:literal),* $(,)?]) => { nbt_inner!([Long; $($lit),*]) };
    ([Long; $($lit:literal),* $(,)?]) => {
        $crate::NbtTag::LongArray(::std::vec![$($lit),*])
    };
    ([I; $($lit:literal),* $(,)?]) => { nbt_inner!([Int; $($lit),*]) };
    ([Int; $($lit:literal),* $(,)?]) => {
        $crate::NbtTag::IntArray(::std::vec![$($lit),*])
    };
    ([$($lit:literal),* $(,)?]) => {
        $crate::NbtTag::List(::std::vec![$($lit.into()),*])
    };
    ([$($t:tt),* $(,)?]) => {
        $crate::NbtTag::List(::std::vec![$($t.into()),*])
    }
}
