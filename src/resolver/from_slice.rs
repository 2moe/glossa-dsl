use alloc::collections::BTreeMap;

use tap::Pipe;

use crate::{
  TemplateResolver,
  error::{ResolverError, ResolverResult},
  parsers::parse_value_or_map_err,
  resolver::TemplateAST,
};

impl TryFrom<&[(&str, &str)]> for TemplateResolver {
  type Error = ResolverError;

  fn try_from(value: &[(&str, &str)]) -> Result<Self, Self::Error> {
    Self::try_from_slice(value)
  }
}

impl<const N: usize> TryFrom<[(&str, &str); N]> for TemplateResolver {
  type Error = ResolverError;

  fn try_from(value: [(&str, &str); N]) -> Result<Self, Self::Error> {
    Self::try_from_str_entries(value.into_iter())
  }
}

impl<K, V> TryFrom<BTreeMap<K, V>> for TemplateResolver
where
  K: AsRef<str>,
  V: AsRef<str>,
{
  type Error = ResolverError;

  fn try_from(value: BTreeMap<K, V>) -> Result<Self, Self::Error> {
    Self::try_from_str_entries(value.into_iter())
  }
}

impl TemplateResolver {
  /// Construct from slice (no_std compatible)
  ///
  /// ```
  /// use tap::Pipe;
  /// use tmpl_resolver::TemplateResolver;
  ///
  /// let res = [
  ///   ("🐱", "喵 ฅ(°ω°ฅ)"),
  ///   ("hello", "Hello {🐱}"),
  /// ]
  ///  .as_ref()
  ///  .pipe(TemplateResolver::try_from_slice)?;
  ///
  /// let text = res.get_with_context("hello", &[])?;
  /// assert_eq!(text, "Hello 喵 ฅ(°ω°ฅ)");
  ///
  /// # Ok::<(), tmpl_resolver::error::ResolverError>(())
  /// ```
  pub fn try_from_slice(raw: &[(&str, &str)]) -> ResolverResult<Self> {
    Self::try_from_str_entries(raw.iter().copied())
  }

  /// Attempts to build a TemplateResolver from raw unprocessed key-value
  /// entries.
  ///
  /// ## Process Flow
  ///
  /// 1. Accepts an iterator of raw (key, value) pairs
  /// 2. Parses each value into template AST (Abstract Syntax Tree)
  /// 3. Converts keys to normalized format
  /// 4. Collects results into a TemplateAST
  /// 5. Constructs the final resolver
  ///
  /// ## Parameters
  /// - `iter`: Iterator over raw unvalidated entries.
  ///   - e.g., `[(k1, v1), (k2, v2)].into_iter()`
  ///
  /// ## Type Constraints
  /// - `K`: Key type with string-like representation
  /// - `V`: Raw value type containing template text
  /// - `I`: Iterator providing raw configuration entries
  ///
  /// ## Example
  ///
  /// ```
  /// # #[cfg(all(feature = "serde", feature = "toml"))] {
  /// use tap::Pipe;
  /// use tmpl_resolver::{TemplateResolver, resolver::MiniStr, resolver::BTreeRawMap};
  ///
  ///
  /// let res = r##"
  ///   meow = "喵"
  ///   "🐱" = "{ meow } ฅ(°ω°ฅ)"
  /// "##
  ///   .pipe(toml::from_str::<BTreeRawMap>)?
  ///   .into_iter()
  ///   .pipe(TemplateResolver::try_from_str_entries)?;
  ///
  /// assert_eq!(res.try_get("🐱")?, "喵 ฅ(°ω°ฅ)");
  ///
  /// # }
  /// # Ok::<(), tmpl_resolver::Error>(())
  /// ```
  ///
  /// See also:
  ///   - [Self::try_from_slice]
  ///   - [Self::try_from_raw]
  pub fn try_from_str_entries<K, V, I>(iter: I) -> ResolverResult<Self>
  where
    K: AsRef<str>,
    V: AsRef<str>,
    I: Iterator<Item = (K, V)>,
  {
    iter
      .map(|(key, value)| {
        parse_value_or_map_err(key.as_ref(), value.as_ref()) //
          .map(|tmpl| (convert_map_key(key.as_ref()), tmpl))
      })
      .collect::<Result<TemplateAST, _>>()?
      .pipe(Self)
      .pipe(Ok)
  }
}

#[cfg(not(feature = "std"))]
fn convert_map_key(key: &str) -> crate::MiniStr {
  key.into()
}

#[cfg(feature = "std")]
fn convert_map_key(key: &str) -> kstring::KString {
  key.pipe(kstring::KString::from_ref)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[ignore]
  #[test]
  fn test_try_from_slice() -> ResolverResult<()> {
    let _res = [
      ("g", "Good"),
      ("greeting", "{g} { time-period }! { $name }"),
      (
        "time-period",
        "$period ->
          [morning] Morning
          *[other] {$period}",
      ),
    ]
    .pipe_as_ref(TemplateResolver::try_from_slice)?;

    // extern crate std;
    // std::dbg!(res);
    Ok(())
  }
}
