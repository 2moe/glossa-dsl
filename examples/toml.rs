//! ```ignore,sh
//! cargo run --package glossa-dsl --example toml --all-features
//! ```
use glossa_dsl::{Resolver, error::ResolverResult, resolver::AHashRawMap};
use tap::{Pipe, Tap};

fn raw_toml_to_hashmap() -> Result<AHashRawMap, toml::de::Error> {
  r##"
    g = "Good"
    time-greeting = """
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
    greeting = "{time-greeting}! { salutation }{ $name }"
  "##
    .pipe(toml::from_str)
}

fn main() -> ResolverResult<()> {
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
