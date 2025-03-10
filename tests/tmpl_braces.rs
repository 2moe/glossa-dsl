use tmpl_resolver::{ResolverResult, TemplateResolver};

#[test]
fn test_tmpl_escaped() -> ResolverResult<()> {
  let resolver: TemplateResolver = [
    ("h", "Hello { $name }"),
    ("greeting", "{h}!{{ how_are_you }}? {{ {$name} }}"),
  ]
  .try_into()?;

  dbg!(&resolver);

  let ctx = [("name", "Alice"), ("how_are_you", "How are you")];

  let result = resolver.get_with_context("greeting", &ctx)?;
  assert_eq!(result, "Hello Alice!how_are_you? {$name}");
  Ok(())
}
