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
  /// Construct from sorted slice (no_std compatible)
  ///
  /// ## Optimization
  ///
  /// - Avoids hashing overhead
  /// - Binary search for efficient lookups
  /// - Automatic sorting ensures search correctness
  pub fn from_raw_slice(raw: &[(&str, &str)]) -> ResolverResult<Self> {
    match raw
      .iter()
      .map(|(key, value)| {
        parse_value_or_map_err(key, value) //
          .map(|tmpl| (convert_map_key(key), tmpl))
      })
      .collect::<Result<TemplateAST, _>>()?
    {
      #[cfg(feature = "std")]
      ast => ast,
      #[cfg(not(feature = "std"))]
      mut ast => {
        ast.sort_unstable_by(|(ka, _), (kb, _)| ka.cmp(kb));
        ast
      }
    }
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
