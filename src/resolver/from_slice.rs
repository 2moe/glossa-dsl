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
    Self::from_raw_slice(&value)
  }
}

impl TemplateResolver {
  /// Construct from slice (no_std compatible)
  ///
  /// ```
  /// use tap::Pipe;
  /// use tmpl_resolver::TemplateResolver;
  ///
  /// let _res = [
  ///   ("g", "Good"),
  ///   ("greeting", "{g} { time-period }! { $name }"),
  ///   (
  ///     "time-period",
  ///     "$period ->
  ///       [morning] Morning
  ///       *[other] {$period}",
  ///   ),
  /// ]
  /// .pipe_as_ref(TemplateResolver::from_raw_slice);
  /// ```
  pub fn from_raw_slice(raw: &[(&str, &str)]) -> ResolverResult<Self> {
    raw
      .iter()
      .map(|(key, value)| {
        parse_value_or_map_err(key, value) //
          .map(|tmpl| (convert_map_key(key), tmpl))
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
