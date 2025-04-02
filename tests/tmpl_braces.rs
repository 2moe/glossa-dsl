use glossa_dsl::{Resolver, error::ResolverResult};

#[test]
fn test_tmpl_escaped() -> ResolverResult<()> {
  let resolver: Resolver = [
    ("h", "Hello { $name }"),
    ("how_are_you", "How Are You"),
    ("greeting", "{h}!{{ how_are_you }}? {{    {$name} }}"),
  ]
  .try_into()?;

  // dbg!(&resolver);

  let ctx = [("name", "Alice")];

  let result = resolver.get_with_context("greeting", &ctx)?;
  assert_eq!(result, "Hello Alice!how_are_you? {$name}");
  Ok(())
}
