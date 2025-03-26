#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]
#![cfg_attr(not(feature = "std"), no_std)]
/*!
# tmpl_resolver

A lightweight template resolution engine with conditional logic support.

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

  let resolver: TemplateResolver = selector_msg.try_into()?;

  let success_msg = resolver.get_with_context("message", &[("status", "success")])?;

  assert_eq!(success_msg, "Operation succeeded!");
  Ok(())
}
```

### Escape

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
    ("greeting", "{h}!{{ how_are_you }}? {{     {$name} }}"),
  ]
  .try_into()?;

  // dbg!(&resolver);

  let ctx = [("name", "Alice")];

  let result = resolver.get_with_context("greeting", &ctx)?;
  assert_eq!(result, "Hello Alice!how_are_you? {$name}");
  Ok(())
}
```
*/
extern crate alloc;

pub mod error;
pub use error::ResolverResult;

mod parsers;
pub(crate) mod part;

pub mod resolver;
pub(crate) use resolver::MiniStr;
pub use resolver::TemplateResolver;

#[cfg(feature = "std")]
pub type ContextMap<'a> = ahash::HashMap<&'a str, &'a str>;
#[cfg(feature = "std")]
pub type ContextMapBuf = ahash::HashMap<kstring::KString, MiniStr>;

pub(crate) mod selector;
pub(crate) mod template;
pub use template::Template;

#[cfg(test)]
#[cfg(not(feature = "std"))]
mod no_std_tests {
  use testutils::simple_benchmark;

  // extern crate std;
  use super::*;
  use crate::error::ResolverResult;

  fn init_ast() -> ResolverResult<resolver::TemplateResolver> {
    [("g", "Good"), ("greeting", "{g} {$period}! { $name }")]
      .as_ref()
      .try_into()
  }

  #[test]
  fn get_text() -> ResolverResult<()> {
    let text = init_ast()?
      .get_with_context("greeting", &[("name", "Tom"), ("period", "Morning")])?;

    assert_eq!(text, "Good Morning! Tom");
    Ok(())
  }

  /// - debug: 5.791µs
  /// - release: 1.958µs
  #[ignore]
  #[test]
  fn bench_no_std_get_text() {
    let ast = init_ast().expect("Failed to init template resolver");

    simple_benchmark(|| {
      ast.get_with_context("greeting", &[("name", "Tom"), ("period", "Morning")])
    });
  }
}

#[cfg(feature = "all")]
#[cfg(test)]
mod tests {
  use std::fs;

  use ahash::HashMap;
  use kstring::KString;
  use testutils::simple_benchmark;

  use super::*;
  use crate::error::ResolverResult;

  fn raw_toml_to_hashmap() -> Result<HashMap<KString, MiniStr>, toml::de::Error> {
    let text = r##"
g = "Good"
time-period = """
$period ->
  [morning] {g} Morning
  [evening] {g} evening
  *[other] {g} {$period}
"""

href = """

<a href=""></a>
end

"""

gender = """

$attr ->
  [male] Mr.
  *[female] Ms.
"""
greeting = "{ time-period }! { gender }{ $name }"
    "##;

    toml::from_str(text)
  }

  #[ignore]
  #[test]
  fn dbg_tomlmap() {
    let _ = dbg!(raw_toml_to_hashmap());
  }

  #[test]
  fn get_text() -> ResolverResult<()> {
    let raw = raw_toml_to_hashmap().expect("Failed to deser toml");
    let resolver = resolver::TemplateResolver::try_from_raw(raw)?;
    let text = resolver.get_with_context(
      "greeting",
      &[
        ("period", "evening"),
        ("name", "Alice"),
        ("attr", "unknown"),
      ],
    )?;
    assert_eq!(text, "Good evening! Ms.Alice");

    Ok(())
  }

  #[ignore]
  fn encode_ast_to_json() -> anyhow::Result<String> {
    let raw = raw_toml_to_hashmap()?;
    let resolver = resolver::TemplateResolver::try_from_raw(raw)?;
    let json_str = serde_json::to_string_pretty(&resolver)?;
    // println!("{toml_str}");
    Ok(json_str)
  }

  #[ignore]
  #[test]
  fn test_serde_bincode_from_json_str() -> anyhow::Result<()> {
    let json_str = encode_ast_to_json()?;
    let data = serde_json::from_str::<resolver::TemplateResolver>(&json_str)?;
    let cfg = bincode::config::standard().with_no_limit();
    let buf = bincode::serde::encode_to_vec(data, cfg)?;
    fs::write("tmp.bincode", &buf)?;
    let (data, n) = bincode::serde::borrow_decode_from_slice::<
      resolver::TemplateResolver,
      _,
    >(&buf, cfg)?;
    dbg!(data, n);
    Ok(())
  }

  #[ignore]
  #[test]
  fn test_deser_bincode_from_file() -> anyhow::Result<()> {
    let cfg = bincode::config::standard();

    let buf = fs::read("tmp.bincode")?;
    let now = std::time::Instant::now();
    let (data, _u) =
      bincode::serde::decode_from_slice::<resolver::TemplateResolver, _>(&buf, cfg)?;
    let elapsed = now.elapsed();
    dbg!(&data);
    eprintln!("elapsed: {elapsed:?}");

    Ok(())
  }

  /// - debug: 6.75µs
  /// - release: 1.541µs
  #[test]
  #[ignore]
  fn bench_resolve() -> anyhow::Result<()> {
    let raw = raw_toml_to_hashmap()?;
    let resolver =
      resolver::TemplateResolver::try_from_raw(raw).expect("Invalid template");
    dbg!(&resolver);

    simple_benchmark(|| {
      resolver.get_with_context(
        "greeting",
        &[
          ("attr", "unknown"),
          ("period", "evening"),
          ("name", "Alice"),
          // ("aa", ""),
          // ("bb", ""),
          // ("cc", ""),
        ],
      )
    });
    Ok(())
  }
}
