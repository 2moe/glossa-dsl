use alloc::boxed::Box;

use crate::{MiniStr, template};

/// Conditional branching structure
///
/// ## Memory Layout
///
/// - Uses Box<[]> for case storage
/// - Optional default branch avoids allocation when unused
///
/// ## Resolution Logic
///
/// 1. Match parameter against case values
/// 2. Use first matching case template
/// 3. Fallback to default if no matches
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Selector {
  /// Parameter name to check
  pub(crate) param: MiniStr,
  /// (Value pattern, Template) pairs
  pub(crate) cases: Box<[(MiniStr, template::Template)]>,
  /// Fallback template when no cases match
  pub(crate) default: Option<Box<template::Template>>,
}
