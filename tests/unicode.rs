#![cfg(feature = "serde")]

// use anyhow::Result as AnyResult;
use glossa_dsl::{Resolver, error::ResolverResult, resolver::BTreeRawMap};
use tap::{Pipe, TryConv};

/// doc test: [Resolver::try_from_str_entries]
#[test]
#[ignore]
fn test_emoji_var() -> ResolverResult<()> {
  let res = r##"
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
    .pipe(toml::from_str::<BTreeRawMap>)?
    // .expect("Failed to deserialize toml")
    // .into_iter()
    // .pipe(Resolver::try_from_str_entries)?;
    .try_conv::<Resolver>()?;

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
  assert_eq!(res.try_get("🐱")?, "喵 ฅ(°ω°ฅ)");

  // dbg!(&text);
  // dbg!(res);
  // dbg!(res.get("🐱"));

  Ok(())
}
