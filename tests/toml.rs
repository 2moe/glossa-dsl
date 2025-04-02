#![cfg(all(feature = "std", feature = "serde"))]
// #![cfg(feature = "std")]

use anyhow::Result as AnyResult;
use tap::{Pipe, Tap};
use glossa_dsl::{
  Resolver, error::ResolverResult, resolver::AHashRawMap,
};

fn raw_toml_to_hashmap() -> Result<AHashRawMap, toml::de::Error> {
  r##"
    g = "Good"
    time-period = """
    $period ->
    [morning] {g} Morning
    [evening] {g} evening
    *[other] {g} {$period}
    """

    salutation = """

    $gender ->
    [male] Mr.
    *[female] Ms.
    """
    greeting = "{ time-period }! { salutation }{ $name }"
  "##
    .pipe(toml::from_str)
}

#[ignore]
#[test]
fn test_new_raw_map() -> AnyResult<()> {
  let resolver: Resolver = raw_toml_to_hashmap()?.try_into()?;

  let text = resolver.get_with_context(
    "greeting",
    &[
      ("period", "evening"),
      ("name", "Alice"),
      ("gender", "unknown"),
    ],
  )?;
  assert_eq!(text, "Good evening! Ms.Alice");
  Ok(())
}

#[test]
fn test_get_with_context() -> ResolverResult<()> {
  let resolver: Resolver = raw_toml_to_hashmap()
    .expect("Failed to deserialize toml str to AHashRawMap")
    .try_into()?;

  let get_text = |ctx| resolver.get_with_context("greeting", ctx);

  [
    ("period", "evening"),
    ("name", "Alice"),
    ("gender", "unknown"),
  ]
  .as_ref()
  .pipe(get_text)?
  .tap(|text| assert_eq!(text, "Good evening! Ms.Alice"));

  [
    ("period", "night"), //
    ("name", "Tom"),
    ("gender", "male"),
  ]
  .as_ref()
  .pipe(get_text)?
  .tap(|text| assert_eq!(text, "Good night! Mr.Tom"));

  let g = resolver.get_with_context("g", &[])?;
  assert_eq!(g, "Good");

  Ok(())
}
