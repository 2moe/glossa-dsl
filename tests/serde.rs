#![cfg(feature = "serde")]
// #![cfg(not(feature = "std"))]
/*!
```ignore,sh
cargo test --package glossa-dsl --test serde --features=serde -- test_ser_no_std --exact --show-output --ignored
```
*/

use glossa_dsl::{Resolver, error::ResolverResult};

#[ignore]
#[test]
fn test_ser_no_std() -> ResolverResult<()> {
  let res: Resolver = [
    ("greeting", "{ time-period }! { salutation }{ $name }"),
    ("salutation", "\n$gender ->\n[male] Mr.\n*[female] Ms.\n"),
    (
      "time-period",
      "$period ->\n[morning] {g} Morning\n[evening] {g} evening\n*[other] {g} {$period}\n",
    ),
    ("g", "Good"),
  ].try_into()?;
  // dbg!(&res);

  let toml_str = toml::to_string_pretty(&res).expect("Invalid data");
  println!("{toml_str}");

  let _data: Resolver =
    toml::from_str(&toml_str).expect("Invalid toml string");

  Ok(())
}
