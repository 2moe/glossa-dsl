use alloc::collections::BTreeMap;

use crate::MiniStr;

#[derive(Debug, Clone)]
pub(crate) enum Context<'a> {
  Empty,
  Slice(&'a [(&'a str, &'a str)]),
  BTree(&'a BTreeMap<MiniStr, MiniStr>),
  #[cfg(feature = "std")]
  Map(&'a crate::ContextMap<'a>),
  #[cfg(feature = "std")]
  MapBuf(&'a crate::ContextMapBuf),
}

impl Default for Context<'_> {
  fn default() -> Self {
    Self::Empty
  }
}

impl<'a> Context<'a> {
  pub(crate) fn get_value(&self, key: &str) -> Option<&str> {
    match self {
      Self::Slice(context) => Self::get_slice_value(context, key),
      Self::BTree(context) => context
        .get(key)
        .map(|v| v.as_str()),
      Self::Empty => None,
      #[cfg(feature = "std")]
      Self::Map(context) => context.get(key).copied(),
      #[cfg(feature = "std")]
      Self::MapBuf(context) => context
        .get(key)
        .map(|v| v.as_str()),
    }
  }

  pub(crate) fn get_slice_value(
    context: &'a [(&str, &str)],
    key: &str,
  ) -> Option<&'a str> {
    context
      .binary_search_by_key(&key, |&(k, _)| k)
      .ok()
      .and_then(|idx| context.get(idx))
      .map(|x| x.1)
  }
}
