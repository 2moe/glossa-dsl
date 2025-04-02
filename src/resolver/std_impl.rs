use core::fmt::Display;
use std::collections::HashMap as StdHashMap;

use kstring::KString;
use tap::Pipe;

use crate::{
  Resolver,
  error::{ResolverError, ResolverResult},
  parsers::parse_value_or_map_err,
  resolver::AST,
};

impl<K, V, S> TryFrom<StdHashMap<K, V, S>> for Resolver
where
  K: Into<KString> + Display,
  V: AsRef<str>,
{
  type Error = ResolverError;

  fn try_from(value: StdHashMap<K, V, S>) -> Result<Self, Self::Error> {
    Self::try_from_raw(value)
  }
}

impl<K, V, S> TryFrom<ahash::AHashMap<K, V, S>> for Resolver
where
  K: Into<KString> + Display,
  V: AsRef<str>,
{
  type Error = ResolverError;

  fn try_from(value: ahash::AHashMap<K, V, S>) -> Result<Self, Self::Error> {
    Self::try_from_raw(value)
  }
}

impl<K, V> TryFrom<Vec<(K, V)>> for Resolver
where
  K: Into<KString> + Display,
  V: AsRef<str>,
{
  type Error = ResolverError;

  fn try_from(value: Vec<(K, V)>) -> Result<Self, Self::Error> {
    Self::try_from_raw(value)
  }
}

impl Resolver {
  /// Construct from `IntoIterator<(K, V)>`, e.g., HashMap
  ///
  /// ## Example
  ///
  /// ```
  /// use glossa_dsl::Resolver;
  /// use tap::pipe::Pipe;
  ///
  /// let resolver = [
  ///    (
  ///      "salutation",
  ///      "
  ///      $gender ->
  ///        [male] Mr.
  ///        *[female] Ms.",
  ///    ),
  ///    ("g", "Good"),
  ///    (
  ///      "time-greeting",
  ///      "$period ->
  ///        [morning] {g} Morning
  ///        [evening] {g} Evening
  ///        *[other] {g} {$period}
  ///      ",
  ///    ),
  ///    ("greeting", "{ time-greeting }! { salutation }{ $name }"),
  ///  ]
  ///  // .into_iter()
  ///  // .map(|(k, v)| (k.into(), v.into()))
  ///  // .collect::<glossa_dsl::resolver::AHashRawMap>()
  ///  .pipe(Resolver::try_from_raw)?;
  ///
  /// let text = resolver
  ///    .get_with_context(
  ///      "greeting",
  ///      &[
  ///        ("period", "evening"),
  ///        ("name", "Alice"),
  ///        ("gender", "unknown"),
  ///      ],
  ///    )
  ///    .expect("Failed to get text");
  ///
  /// assert_eq!(text, "Good Evening! Ms.Alice");
  /// # Ok::<(), glossa_dsl::error::ResolverError>(())
  /// ```
  pub fn try_from_raw<K, V, I>(iter: I) -> ResolverResult<Self>
  where
    K: Into<KString> + Display,
    V: AsRef<str>,
    I: IntoIterator<Item = (K, V)>,
  {
    iter
      .into_iter()
      .map(|(key, value)| {
        parse_value_or_map_err(&key, value.as_ref()) //
          .map(|tmpl| (key.into(), tmpl))
      })
      // .tap_dbg(|x| println!("{:?}", x.size_hint()))
      .collect::<Result<AST, _>>()?
      .pipe(Self)
      .pipe(Ok)
  }
}
