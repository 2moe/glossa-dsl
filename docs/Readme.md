# tmpl_resolver

A lightweight template resolution engine with conditional logic support.

Its syntax is similar to Mozilla Fluent, but it doesn't have as many features as Fluent.

The parser is implemented using `nom` and can be used in `no_std`.

[![tmpl-resolver.crate](https://img.shields.io/crates/v/tmpl-resolver)](https://crates.io/crates/tmpl-resolver)

[![Documentation](https://docs.rs/tmpl-resolver/badge.svg)](https://docs.rs/tmpl-resolver)

[![Apache-2 licensed](https://img.shields.io/crates/l/tmpl-resolver.svg)](../License)

<!--
## Core Types

- [`TemplateResolver`]: Main resolution engine

### Private Types

- [`Template`]: Enum representing template variants
- [`Selector`]: Conditional branching structure
- [`ResolverError`]: Comprehensive error reporting
-->

## Key Concepts

- **Templates**: Contain either direct text parts or conditional selectors
- **Selectors**: Enable branch logic based on parameter values
- **Variable Resolution**: Recursive resolution with context-aware lookup

## Features

- `[]`
  - Minimal configuration for `no_std` use
- ["all"]: Enable all features
- ["std"]
  - Enables standard library
  - Uses ahash::HashMap for faster lookups
- ["serde"]
  - Adds serialization capabilities
  - Enables template storage/transmission
- ["bincode"]
  - Efficient binary serialization

## Examples

### Quick Start

```sh
cargo add toml
cargo add tmpl-resolver --features=std,serde
```

```rust
use tmpl_resolver::{TemplateResolver, resolver::AHashRawMap, error::ResolverResult};

fn raw_toml_to_hashmap() -> Result<AHashRawMap, toml::de::Error> {
  let text = r##"
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
  "##;

  toml::from_str(text)
}

fn main() -> ResolverResult<()> {
  let resolver: TemplateResolver = raw_toml_to_hashmap()
    .expect("Failed to deserialize toml str to AHashRawMap")
    .try_into()?;

 let text = resolver.get_with_context(
    "greeting",
    &[
      ("period", "evening"),
      ("name", "Alice"),
      ("gender", "unknown"),
    ],
  )?;
  assert_eq!(text, "Good evening! Ms.Alice");

  let text = resolver.get_with_context(
    "greeting",
    &[
      ("period", "night"),
      ("name", "Tom"),
      ("gender", "male"),
    ],
  )?;
  assert_eq!(text, "Good night! Mr.Tom");

  let g = resolver.get_with_context("g", &[])?;
  assert_eq!(g, "Good");

  Ok(())
}
```

### Basic

```rust
use tmpl_resolver::{TemplateResolver, error::ResolverResult};

fn main() -> ResolverResult<()> {
  let resolver: TemplateResolver = [
      ("h", "Hello"),
      ("greeting", "{h} { $name }! Today is {$day}")
    ]
    .try_into()?;

  let result = resolver.get_with_context("greeting", &[("name", "Alice"), ("day", "Monday")])?;
  assert_eq!(result, "Hello Alice! Today is Monday");
  Ok(())
}
```

### Conditional Logic

```rust
use tmpl_resolver::{TemplateResolver, error::ResolverResult};

fn main() -> ResolverResult<()> {
  let selector_msg = [(
    "message",
    r#"$status ->
      [success] Operation succeeded!
      [error] Error occurred!
      *[default] Unknown status: {$status}
    "#
  )];

  let resolver = TemplateResolver::from_raw_slice(&selector_msg)?;

  let success_msg = resolver.get_with_context("message", &[("status", "success")])?;

  assert_eq!(success_msg, "Operation succeeded!");
  Ok(())
}
```
