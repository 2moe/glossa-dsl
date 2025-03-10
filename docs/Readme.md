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

### Basic

```rust
use tmpl_resolver::{TemplateResolver, error::ResolverResult};

fn main() -> ResolverResult<()> {
  let resolver: TemplateResolver = [
      ("h", "Hello"),
      ("greeting", "{h} { $name }! Today is {$day}.")
    ]
    .try_into()?;

  let ctx = [("name", "Alice"), ("day", "Sunday")];

  let result = resolver.get_with_context("greeting", &ctx)?;
  assert_eq!(result, "Hello Alice! Today is Sunday.");
  Ok(())
}
```

### Conditional Logic

```rust
use tmpl_resolver::{TemplateResolver, error::ResolverResult};

fn main() -> ResolverResult<()> {
  let selector_msg = [(
    "message",
    r#"
    $status ->
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

## Escape

- `"{{ a   }}"` => `"a"`
- `"{{{a}}}"` => `"a"`
- `"{{{{  a  }}}}"` => `"a"`
- `"{{    {a}    }}"` => `"{a}"`
- `"{{a}"` => ❌ nom Error, code: take_until
- `"{{{    {{a}}    }}}"` => `"{{a}}"`
- `"{{{    {{ a }}    }}}"` => `"{{ a }}"`
- `"{{{ {{a} }}}"` => `"{{a}"`

```rust
use tmpl_resolver::{ResolverResult, TemplateResolver};

fn main() -> ResolverResult<()> {
  let resolver: TemplateResolver = [
    ("h", "Hello { $name }"),
    ("how_are_you", "How Are You"),
    ("greeting", "{h}!{{ how_are_you }}? {{    {$name} }}"),
  ]
  .try_into()?;

  let ctx = [("name", "Alice")];

  let result = resolver.get_with_context("greeting", &ctx)?;
  assert_eq!(result, "Hello Alice!how_are_you? {$name}");
  Ok(())
}
```

### toml

```sh
cargo add toml tap
cargo add tmpl-resolver --features=std,serde
```

```rust
use tmpl_resolver::{TemplateResolver, resolver::AHashRawMap, ResolverResult};
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
  let resolver: TemplateResolver = raw_toml_to_hashmap()
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
```

### Unicode

We can use emoji as variable name.

```toml
"🐱" = "ฅ(°ω°ฅ)"
hello = "Hello {🐱}"
```

For example, when `hello` references `{🐱}`, after expanding `hello`, we would get `"Hello ฅ(°ω°ฅ)"`.

---

```rust
use tap::Pipe;
use tmpl_resolver::{ResolverResult, TemplateResolver, resolver::AHashRawMap};

fn main() -> ResolverResult<()> {
  let res: TemplateResolver = r##"
      "🐱" = "喵 ฅ(°ω°ฅ)"

      "问候" = """
        $period ->
          [morning] 早安{🐱}
          [night] 晚安{🐱}
          *[other] {$period}好
      """

      "称谓" = """
      $gender ->
        *[male] 先生
        [female] 女士
      """

      greeting = "{ 问候 }！{ $name }{ 称谓 }。"
    "##
    .pipe(toml::from_str::<AHashRawMap>)
    .expect("Failed to deserialize toml")
    .try_into()?;

  let get_text = |ctx| res.get_with_context("greeting", ctx);

  let text = [
    ("period", "morning"),
    ("name", "Young"),
    ("gender", "unknown"),
  ]
  .as_ref()
  .pipe(get_text)?;

  assert_eq!(text, "早安喵 ฅ(°ω°ฅ)！Young先生。");
  assert_eq!(res.get_with_context("🐱", &[])?, "喵 ฅ(°ω°ฅ)");

  Ok(())
}
```
