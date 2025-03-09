use tap::Tap;
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
    let template = self
      .get_value_by_key(var_name)
      .ok_or_else(|| ResolverError::UndefinedVariable(var_name.into()))?;

    let sorted_context = context
      .iter()
      .copied()
      .collect::<TinyVec<[(&str, &str); 5]>>()
      .tap_mut(|x| x.sort_unstable_by_key(|&(k, _)| k));

    self.process_template(template, &Context::Slice(&sorted_context))
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
    let template = self
      .get_value_by_key(var_name)
      .ok_or_else(|| ResolverError::UndefinedVariable(var_name.into()))?;

    self.process_template(template, &Context::Map(context_map))
  }

  pub(crate) fn get_value_by_key(&self, key: &str) -> Option<&Template> {
    self.0.get(key)
  }

  // old: Previously, for no_std, Vec<MiniStr, Template> was used as the structure
  // for the AST. Since version 0.0.3, BTreeMap is used, so the following
  // function is deprecated.
  //
  // #[cfg(not(feature = "std"))]
  // pub(crate) fn get_value_by_key(&self, key: &str) -> Option<&Template> {
  //   let vec = &self.0;
  //
  //   vec
  //     .binary_search_by_key(&key, |(k, _)| k)
  //     .ok()
  //     .and_then(|idx| vec.get(idx))
  //     .map(|x| &x.1)
  // }
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
