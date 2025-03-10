//! Simple L10n Message Example
//!
//! ```ignore,sh
//! cargo run --package tmpl-resolver --example unread --all-features
//! ```
#![cfg(all(feature = "std", feature = "serde"))]

use anyhow::Result as AnyResult;
use tap::{Pipe, TryConv};
use tmpl_resolver::{TemplateResolver, resolver::AHashRawMap};

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

const EN_TOML: &str = r#"
  num-to-en = """
    $num ->
      [0] zero
      [1] one
      [2] two
      [3] three
      *[other] {$num}
  """

  unread_msg = "unread message"

  unread-count = """
    $num ->
      [0] No {unread_msg}s.
      [1] You have { num-to-en } {unread_msg}.
      *[other] You have { num-to-en } {unread_msg}s.
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
