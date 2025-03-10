mod branch;
pub(crate) mod context;
mod process_tmpl;

use alloc::{boxed::Box, vec::Vec};

use compact_str::format_compact;
use nom::{
  IResult, Parser,
  bytes::complete::{tag, take_while1},
  character::complete::multispace0,
  multi::many0,
};
use tap::Pipe;

use crate::{
  error::{ResolverError, ResolverResult},
  selector, template,
};

pub(crate) fn parse_value_or_map_err<D: core::fmt::Display>(
  key: D,
  value: &str,
) -> ResolverResult<template::Template> {
  parse_value(value).map_err(|e| {
    format_compact!("Failed to parse '{key}': {e}") //
      .pipe(ResolverError::ParseError)
  })
}

fn parse_value(input: &str) -> ResolverResult<template::Template> {
  match parse_conditional(input.trim_ascii()) {
    Ok((_, cond)) => cond.pipe(template::Template::Conditional),
    _ => template::parse_template(input)?.pipe(template::Template::Parts),
  }
  .pipe(Ok)
}

fn parse_conditional(input: &str) -> IResult<&str, selector::Selector> {
  let (input, _) = tag("$").parse(input)?;
  let (input, param) =
    take_while1(|c: char| c.is_alphanumeric() || c == '-' || c == '_')
      .parse(input)?;

  let (input, _t) = (multispace0, tag("->"), multispace0).parse(input)?;

  let (input, branches) = many0(branch::parse_branch).parse(input)?;

  let (cases, default) = branches //
    .into_iter()
    .fold(
      (Vec::with_capacity(8), None),
      |(mut cases, mut default), branch| {
        match branch.is_default {
          true => {
            default = branch
              .template
              .pipe(Box::new)
              .pipe(Some)
          }
          _ => cases.push((branch.value, branch.template)),
        }
        (cases, default)
      },
    );

  (
    input,
    selector::Selector {
      param: param.into(),
      cases: cases.into(),
      default,
    },
  )
    .pipe(Ok)
}
