use compact_str::ToCompactString;
use nom::{
  IResult, Parser,
  bytes::complete::{tag, take_till, take_until},
  sequence::delimited,
};
use tap::Pipe;
use tinyvec::TinyVec;

use crate::{
  error::{ResolverError, ResolverResult},
  part::{TemplatePart, VariableRef},
  selector,
};
pub(crate) type TinyTemplateParts = TinyVec<[TemplatePart; 5]>;

/// Core template representation
///
/// ## Variants
/// - Conditional: Enables branching logic based on parameters
/// - Parts: Direct template content (text + variables)
///
/// ## Serialization
/// - Derives Serialize/Deserialize with serde feature
/// - Uses compact binary representation with bincode
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Template {
  /// Conditional template branch
  Conditional(selector::Selector),
  /// Linear template segments
  Parts(TinyTemplateParts),
}

impl Default for Template {
  fn default() -> Self {
    Self::Parts(Default::default())
  }
}

#[allow(clippy::unnecessary_lazy_evaluations)]
pub(crate) fn parse_template(input: &str) -> ResolverResult<TinyTemplateParts> {
  let mut remaining = input;

  core::iter::from_fn(|| {
    (!remaining.is_empty()).then(|| ())?;

    match parse_variable(remaining) {
      Ok((next, var)) => {
        remaining = next;
        var
          .pipe(TemplatePart::Variable)
          .pipe(Ok)
          .into()
      }
      Err(_) => parse_text(remaining)
        .map(|(next, text)| {
          remaining = next;
          match text.is_empty() {
            true => None,
            _ => text
              .to_compact_string()
              .pipe(TemplatePart::Text)
              .into(),
          }
        })
        .map_err(|e| {
          e.to_compact_string()
            .pipe(ResolverError::ParseError)
        })
        .transpose(),
    }
  })
  .collect()
}

fn parse_variable(input: &str) -> IResult<&str, VariableRef> {
  // let (input, _) = tag("{").parse(input)?;
  // let (input, content) = take_until("}").parse(input)?;
  // let (input, _) = tag("}").parse(input)?;
  let (input, content) =
    delimited(tag("{"), take_until("}"), tag("}")).parse(input)?;
  let content = content.trim();

  match content.strip_prefix('$') {
    Some(param) => (input, VariableRef::Parameter(param.trim().into())),
    _ => (input, VariableRef::Variable(content.into())),
  }
  .pipe(Ok)
}

fn parse_text(input: &str) -> IResult<&str, &str> {
  take_till(|c| c == '{').parse(input)
}
