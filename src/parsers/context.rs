#[derive(Debug, Clone)]
pub(crate) enum Context<'a> {
  Slice(&'a [(&'a str, &'a str)]),
  Empty,
  #[cfg(feature = "std")]
  Map(&'a crate::ContextMap<'a>),
}

impl<'a> Context<'a> {
  pub(crate) fn get_value(&self, key: &str) -> Option<&str> {
    match self {
      Self::Slice(context) => Self::get_slice_value(context, key),
      Self::Empty => None,
      #[cfg(feature = "std")]
      Self::Map(context) => context.get(key).copied(),
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
