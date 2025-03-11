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
    Self::from_raw_slice(value)
  }
}

impl<const N: usize> TryFrom<[(&str, &str); N]> for TemplateResolver {
  type Error = ResolverError;

  fn try_from(value: [(&str, &str); N]) -> Result<Self, Self::Error> {
    Self::try_from_raw_entries_iter(value.into_iter())
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
  ///  .pipe(TemplateResolver::from_raw_slice)?;
  ///
  /// let text = res.get_with_context("hello", &[])?;
  /// assert_eq!(text, "Hello 喵 ฅ(°ω°ฅ)");
  ///
  /// # Ok::<(), tmpl_resolver::error::ResolverError>(())
  /// ```
  pub fn from_raw_slice(raw: &[(&str, &str)]) -> ResolverResult<Self> {
    Self::try_from_raw_entries_iter(raw.iter().copied())
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
  /// See also:
  ///   - [Self::from_raw_slice]
  ///   - [Self::from_raw]
  pub fn try_from_raw_entries_iter<K, V, I>(iter: I) -> ResolverResult<Self>
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
  fn test_from_raw_slice() -> ResolverResult<()> {
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
    .pipe_as_ref(TemplateResolver::from_raw_slice)?;

    // extern crate std;
    // std::dbg!(res);
    Ok(())
  }
}
