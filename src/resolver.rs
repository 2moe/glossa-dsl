#[cfg(feature = "bincode")]
#[cfg(feature = "std")]
mod bin_code;

#[cfg(feature = "bincode")]
pub(crate) mod bin_code_nostd;

mod from_slice;
mod lookup_value;
mod ordered_map;

#[cfg(feature = "std")]
mod std_impl;

use alloc::collections::BTreeMap;

/// Compact string type optimized for small string storage.
/// - Uses stack storage for strings <= 24 bytes (for 64-bit sys).
/// - Fallback to heap allocation for longer strings.<hr>
pub use compact_str::CompactString as MiniStr;
//
#[cfg(feature = "std")]
pub use kstring::KString;

use crate::template::Template;

#[cfg(feature = "std")]
pub type AHashRawMap = ahash::HashMap<KString, MiniStr>;

pub type BTreeRawMap = BTreeMap<MiniStr, MiniStr>;

#[cfg(feature = "std")]
pub type AST = ahash::HashMap<KString, Template>;

#[cfg(feature = "std")]
pub type OrderedAST = BTreeMap<KString, Template>;

#[cfg(not(feature = "std"))]
pub type OrderedAST = BTreeMap<MiniStr, Template>;

#[cfg(not(feature = "std"))]
pub type AST = OrderedAST;

/// Main template resolution engine
///
/// ## Implementation Notes
///
/// - `cfg(feature = "std")`
///   - Uses HashMap with std for O(1) lookups
/// - no_std:
///   - Falls back to BTreeMap in no_std (O(log n) lookups)
#[derive(Default, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Resolver(pub AST);

impl core::ops::Deref for Resolver {
  type Target = AST;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
