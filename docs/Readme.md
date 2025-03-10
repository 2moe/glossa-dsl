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

## Basic

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

## Conditional Logic

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

## Real World Examples

Add dependencies

```sh
cargo add toml tap anyhow
cargo add tmpl-resolver --features=std,serde
```

### Emoji

We can use emoji as ~~variable~~ identifier name.

---

toml:

```toml
"🐱" = "ฅ(°ω°ฅ)"
hello = "Hello {🐱}"
```

1. `hello` references `{🐱}`
2. expanding `hello`
3. we would get `"Hello ฅ(°ω°ฅ)"`.

rust:

```rust
let text = res.get_with_context("hello", &[])?;
assert_eq!(text, "Hello ฅ(°ω°ฅ)");
```

---

toml:

```toml
hello = "Hello {$🐱}"
```

> `$🐱` means that its value is passed in externally.

rust:

```rust
let text = res.get_with_context("hello", &[("🐱", "QwQ")])?;
assert_eq!(text, "Hello QwQ");
```

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

### Simple L10n Message

```rust
use anyhow::Result as AnyResult;
use tap::{Pipe, TryConv};
use tmpl_resolver::{TemplateResolver, resolver::AHashRawMap};

const EN_TOML: &str = r#"
  num-to-en = """
    $num ->
      [0] zero
      [1] one
      [2] two
      [3] three
      *[other] {$num}
  """

  unread = "unread message"

  unread-count = """
    $num ->
      [0] No {unread}s.
      [1] You have { num-to-en } {unread}.
      *[other] You have { num-to-en } {unread}s.
  """

  show-unread-messages-count = "{unread-count}"
"#;

const ZH_TOML: &str = r#"
  "阿拉伯数字转汉字" = """
    $num ->
      [0] 〇
      [1] 一
      [2] 二
      [3] 三
      *[其他] {$num}
  """

  "未读msg" = "未读消息"

  "显示未读消息数量" = """
    $num ->
        [0] 没有{ 未读msg }。
        [2] 您有两条{ 未读msg }。
       *[其他] 您有{ 阿拉伯数字转汉字 }条{ 未读msg }。
  """

  show-unread-messages-count = "{显示未读消息数量}"
"#;

fn main() -> AnyResult<()> {
  let get_text = |lang| -> AnyResult<_> {
    match lang {
      "zh" => ZH_TOML,
      _ => EN_TOML,
    }
    .pipe(toml::from_str::<AHashRawMap>)?
    .try_conv::<TemplateResolver>()?
    .pipe(|r| {
      move |num_str| {
        r.get_with_context("show-unread-messages-count", &[("num", num_str)])
      }
    })
    .pipe(Ok)
  };

  let get_en_text = get_text("en")?;
  assert_eq!(get_en_text("0")?, "No unread messages.");
  assert_eq!(get_en_text("1")?, "You have one unread message.");
  assert_eq!(get_en_text("2")?, "You have two unread messages.");
  assert_eq!(get_en_text("100")?, "You have 100 unread messages.");

  let get_zh_text = get_text("zh")?;
  assert_eq!(get_zh_text("0")?, "没有未读消息。");
  assert_eq!(get_zh_text("1")?, "您有一条未读消息。");
  assert_eq!(get_zh_text("2")?, "您有两条未读消息。");
  assert_eq!(get_zh_text("100")?, "您有100条未读消息。");

  Ok(())
}
```
