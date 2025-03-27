use tmpl_resolver::{TemplateResolver, error::ResolverResult};

#[test]
fn test_resolver_from_slice() -> ResolverResult<()> {
  let resolver: TemplateResolver =
    [("h", "Hello"), ("greeting", "{h} { $name }!")].try_into()?;

  let ctx = [("name", "Alice")];

  let result = resolver.get_with_context("greeting", &ctx)?;
  assert_eq!(result, "Hello Alice!");
  Ok(())
}
