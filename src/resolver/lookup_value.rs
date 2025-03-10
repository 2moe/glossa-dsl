use tap::{Pipe, Tap};
use tinyvec::TinyVec;

// use super::{ResolverResult, TemplateResolver};
use crate::{
  MiniStr,
  error::{ResolverError, ResolverResult},
  parsers::context::Context,
  resolver::TemplateResolver,
  template::Template,
};

impl TemplateResolver {
  /// Core resolution method
  ///
  /// ## Algorithm
  ///
  /// 1. Context sorting for O(log n) parameter lookups
  /// 2. Recursive template evaluation
  /// 3. Branch prediction for conditional blocks
  ///
  /// ## Example
  ///
  /// ```
  /// use tmpl_resolver::TemplateResolver;
  ///
  /// let res: TemplateResolver = [
  ///   ("g", "Good"),
  ///   ("greeting", "{g} { time-period }! { $name }"),
  ///   (
  ///     "time-period",
  ///     "$period ->
  ///       [morning] Morning
  ///       *[other] {$period}",
  ///   ),
  /// ]
  /// .try_into()?;
  ///
  /// let ctx = [("name", "Tom"), ("period", "night")];
  ///
  /// let text = res.get_with_context("greeting", &ctx)?;
  /// assert_eq!(text, "Good night! Tom");
  ///
  /// # Ok::<(), tmpl_resolver::error::ResolverError>(())
  /// ```
  ///
  /// See also: [Self::get_with_ctx_map],
  pub fn get_with_context(
    &self,
    var_name: &str,
    context: &[(&str, &str)],
  ) -> ResolverResult<MiniStr> {
    let process = |ctx| self.try_get_template_and_process(var_name, ctx);

    match context.is_empty() {
      true => return process(&Context::Empty),
      _ => context
        .iter()
        .copied()
        .collect::<TinyVec<[(&str, &str); 5]>>()
        .tap_mut(|x| x.sort_unstable_by_key(|&(k, _)| k)),
    }
    .as_ref()
    .pipe(Context::Slice)
    .pipe_ref(process)
  }

  #[cfg(feature = "std")]
  /// Similar to [Self::get_with_context], but the context is
  /// [`&ContextMap`](crate::ContextMap) instead of `&[(&str, &str)]`.
  ///
  /// ## Example
  ///
  /// ```
  /// use tmpl_resolver::TemplateResolver;
  ///
  /// let res: TemplateResolver = [
  ///   ("g", "Good"),
  ///   ("greeting", "{g} { time-period }! { $name }"),
  ///   (
  ///     "time-period",
  ///     "$period ->
  ///       [morning] Morning
  ///       *[other] {$period}",
  ///   ),
  /// ]
  /// .try_into()?;
  ///
  /// let ctx_map = [("name", "Tom"), ("period", "night")]
  ///   .into_iter()
  ///   .collect();
  ///
  /// let text = res.get_with_ctx_map("greeting", &ctx_map)?;
  /// assert_eq!(text, "Good night! Tom");
  ///
  /// # Ok::<(), tmpl_resolver::error::ResolverError>(())
  /// ```
  pub fn get_with_ctx_map(
    &self,
    var_name: &str,
    context_map: &crate::ContextMap,
  ) -> ResolverResult<MiniStr> {
    let process = |ctx| self.try_get_template_and_process(var_name, ctx);

    match context_map.is_empty() {
      true => return process(&Context::Empty),
      _ => context_map.pipe(Context::Map),
    }
    .pipe_ref(process)
  }

  pub(crate) fn try_get_template(&self, key: &str) -> ResolverResult<&Template> {
    self
      .0
      .get(key)
      .ok_or_else(|| ResolverError::UndefinedVariable(key.into()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(feature = "std")]
  #[ignore]
  #[test]
  fn test_get_with_ctx_map() -> ResolverResult<()> {
    let res: TemplateResolver = [
      ("g", "Good"),
      ("greeting", "{g} { time-period }! { $name }"),
      (
        "time-period",
        "$period ->
          [morning] Morning
          *[other] {$period}",
      ),
    ]
    .try_into()?;

    let ctx_map = [("name", "Tom"), ("period", "night")]
      .into_iter()
      .collect();

    let text = res.get_with_ctx_map("greeting", &ctx_map)?;
    assert_eq!(text, "Good night! Tom");
    Ok(())
  }
}
