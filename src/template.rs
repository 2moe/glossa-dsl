use compact_str::ToCompactString;
use nom::{
  IResult, Parser,
  bytes::complete::{tag, take_till, take_until, take_while},
  sequence::delimited,
};
use tap::Pipe;
use tinyvec::TinyVec;

use crate::{
  MiniStr,
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
              .pipe(MiniStr::from)
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
  // => escaped text, not variable
  if input.starts_with("{{") {
    return nom::error::Error::new(input, nom::error::ErrorKind::Verify)
      .pipe(nom::Err::Error)
      .pipe(Err);
  }

  let (input, content) =
    delimited(tag("{"), take_until("}"), tag("}")).parse(input)?;
  let content = content.trim();

  match content.strip_prefix('$') {
    Some(param) => (input, VariableRef::Parameter(param.trim().into())),
    _ => (input, VariableRef::Variable(content.into())),
  }
  .pipe(Ok)
}

fn parse_delimited_braces(input: &str) -> IResult<&str, &str> {
  // Count opening braces
  let (input, braces) = take_while(|c| c == '{').parse(input)?;
  let n = braces.len();

  // "{{" => n=2
  // "{{{" => n=3
  // if n=2 => "}}"
  //    n=3 => "}}}"
  let closing_pattern = '}'
    .pipe(core::iter::once)
    .cycle()
    .take(n)
    .collect::<MiniStr>();

  // Extract content until closing pattern
  let (input, content) = closing_pattern
    .pipe_deref(take_until)
    .parse(input)?;

  // Verify and consume closing braces
  let (input, _) = closing_pattern
    .pipe_deref(tag)
    .parse(input)?;

  Ok((input, content.trim_ascii()))
}

fn parse_text(input: &str) -> IResult<&str, &str> {
  let (input, content) = take_till(|c| c == '{').parse(input)?;

  match [input.starts_with("{{"), content.is_empty()]
    .iter()
    .any(|b| !b)
  {
    // input.not_starts_with("{{") or content.is_not_empty()
    true => Ok((input, content)),
    _ => parse_delimited_braces(input),
  }
}
