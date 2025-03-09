use nom::{
  IResult, Parser,
  bytes::complete::{tag, take_till, take_until},
  character::complete::multispace0,
  combinator::opt,
  sequence::delimited,
};
use tap::Pipe;

use crate::{MiniStr, parsers, template};

#[derive(Debug, Clone)]
pub(crate) struct Branch {
  pub(crate) is_default: bool,
  pub(crate) value: MiniStr,
  pub(crate) template: template::Template,
}

pub(crate) fn parse_branch(input: &str) -> IResult<&str, Branch> {
  let (input, _) = multispace0(input)?;
  let (input, is_default) = opt(tag("*")).parse(input)?;
  let is_default = is_default.is_some();

  // let (input, _) = tag("[").parse(input)?;
  // let (input, value) = take_until("]").parse(input)?;
  // let (input, _) = tag("]").parse(input)?;
  let (input, value) =
    delimited(tag("["), take_until("]"), tag("]")).parse(input)?;

  let (input, _) = multispace0(input)?;
  let (input, content) = take_till(|c| c == '\n' || c == '\r').parse(input)?;

  let template = parsers::parse_value(content).map_err(|_e| {
    nom::error::Error::new(content, nom::error::ErrorKind::Verify)
      .pipe(nom::Err::Error)
  })?;

  Ok((
    input,
    Branch {
      is_default,
      value: value.trim().into(),
      template,
    },
  ))
}
