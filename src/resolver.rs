#[cfg(feature = "serde")]
mod serialize;

#[cfg(feature = "bincode")]
mod bin_code;

//
mod from_slice;
mod lookup_value;

#[cfg(feature = "std")]
mod std_impl;

/// Compact string type optimized for small string storage.
/// - Uses stack storage for strings <= 24 bytes (for 64-bit sys).
/// - Fallback to heap allocation for longer strings.<hr>
pub use compact_str::CompactString as MiniStr;
//
#[cfg(feature = "std")]
pub use kstring::KString;

//
use crate::template::Template;

#[cfg(feature = "std")]
pub type AHashRawMap = ahash::HashMap<KString, MiniStr>;

#[cfg(feature = "std")]
pub type TemplateAST = ahash::HashMap<KString, Template>;

#[cfg(not(feature = "std"))]
pub type TemplateAST = alloc::vec::Vec<(MiniStr, Template)>;

/// Main template resolution engine
///
/// ## Implementation Notes
///
/// - `cfg(feature = "std")`
///   - Uses HashMap with std for O(1) lookups
/// - no_std:
///   - Falls back to sorted Vec in no_std (O(log n) lookups)
///   - Automatic sorting in no_std maintains lookup efficiency
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct TemplateResolver(TemplateAST);

impl core::ops::Deref for TemplateResolver {
  type Target = TemplateAST;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
