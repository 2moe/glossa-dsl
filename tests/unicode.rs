#![cfg(feature = "std")]
// use anyhow::Result as AnyResult;
use tap::Pipe;
use tmpl_resolver::{ResolverResult, TemplateResolver, resolver::AHashRawMap};

#[test]
fn test_emoji_var() -> ResolverResult<()> {
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
        [male] 先生
        [female] 女士
        *[test] { $🧑‍🏫 }
      """

      greeting = "{ 问候 }！{ $name }{ 称谓 }。"
    "##
    .pipe(toml::from_str::<AHashRawMap>)
    .expect("Failed to deserialize toml")
    .try_into()?;

  let get_text = |ctx| res.get_with_context("greeting", ctx);

  let text = [
    ("🧑‍🏫", "🧑🧑‍🏫"),
    ("period", "morning"),
    ("name", "Young"),
    ("gender", "unknown"),
  ]
  .as_ref()
  .pipe(get_text)?;

  assert_eq!(text, "早安喵 ฅ(°ω°ฅ)！Young🧑🧑‍🏫。");
  assert_eq!(res.get_with_context("🐱", &[])?, "喵 ฅ(°ω°ฅ)");

  // dbg!(&text);
  // dbg!(res);
  // dbg!(res.get("🐱"));

  Ok(())
}
