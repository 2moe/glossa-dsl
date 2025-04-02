use tap::{Pipe, Tap};
use tinyvec::TinyVec;

// use super::{ResolverResult, Resolver};
use crate::{
  MiniStr,
  error::{ResolverError, ResolverResult},
  parsers::context::Context,
  resolver::{BTreeRawMap, Resolver},
  template::Template,
};

impl Resolver {
  /// Core resolution method
  ///
  /// > If the context is empty, you can directly use [Self::try_get].
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
  /// use glossa_dsl::{Resolver, error::ResolverError};
  ///
  /// let res: Resolver = [
  ///   ("h", "Hello"),
  ///   ("greeting", "{h} {$🐱}"),
  /// ]
  /// .try_into()?;
  ///
  /// let ctx = [("🐱", "喵 ฅ(°ω°ฅ)")];
  ///
  /// let text = res.get_with_context("greeting", &ctx)?;
  /// assert_eq!(text, "Hello 喵 ฅ(°ω°ฅ)");
  ///
  /// # Ok::<(), ResolverError>(())
  /// ```
  ///
  /// See also: [Self::get_with_ctx_map]
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

  /// Similar to [Self::get_with_context], but the context is
  /// `BTreeMap<MiniStr, MiniStr>` instead of `&[(&str, &str)]`.
  pub fn get_with_ctx_btree_map(
    &self,
    var_name: &str,
    context_map: &BTreeRawMap,
  ) -> ResolverResult<MiniStr> {
    let process = |ctx| self.try_get_template_and_process(var_name, ctx);

    match context_map.is_empty() {
      true => Context::Empty,
      _ => context_map.pipe(Context::BTree),
    }
    .pipe_ref(process)
  }

  ///  Similar to [Self::get_with_context], but no context.
  ///
  /// ## Example
  ///
  /// ```
  /// use glossa_dsl::{Resolver, error::ResolverError};
  ///
  /// let res: Resolver = [
  ///   ("🐱", "ฅ(°ω°ฅ)"),
  ///   ("hi", "Hello"),
  ///   ("greeting", "{ hi } { 🐱 }"),
  /// ]
  /// .try_into()?;
  ///
  /// let text = res.try_get("greeting")?;
  /// assert_eq!(text, "Hello ฅ(°ω°ฅ)");
  ///
  /// # Ok::<(), ResolverError>(())
  /// ```
  pub fn try_get(&self, var_name: &str) -> ResolverResult<MiniStr> {
    let process = |ctx| self.try_get_template_and_process(var_name, ctx);
    process(&Context::Empty)
  }

  #[cfg(feature = "std")]
  /// Similar to [Self::get_with_context], but the context is
  /// [`&ContextMap`](crate::ContextMap) instead of `&[(&str, &str)]`.
  ///
  /// > If the parameter you need to pass is a Context HashMap that owns the
  /// > data internally (e.g., `HashMap<KString, CompactString>`), instead of
  /// > `HashMap<&str, &str>`, please use [Self::get_with_ctx_map_buf].
  ///
  /// ## Example
  ///
  /// ```
  /// use glossa_dsl::Resolver;
  ///
  /// let res: Resolver = [
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
  /// # Ok::<(), glossa_dsl::error::ResolverError>(())
  /// ```
  pub fn get_with_ctx_map(
    &self,
    var_name: &str,
    context_map: &crate::ContextMap,
  ) -> ResolverResult<MiniStr> {
    let process = |ctx| self.try_get_template_and_process(var_name, ctx);

    match context_map.is_empty() {
      true => Context::Empty,
      _ => context_map.pipe(Context::Map),
    }
    .pipe_ref(process)
  }

  #[cfg(feature = "std")]
  /// Similar to [Self::get_with_ctx_map], but the context is
  /// [`&ContextMapBuf`](crate::ContextMapBuf) instead of
  /// [`&ContextMap`](crate::ContextMap).
  ///
  /// ## Example
  ///
  /// ```
  /// use glossa_dsl::Resolver;
  ///
  /// let res: Resolver = [
  ///   ("greeting", "{$hi} { $name }"),
  /// ]
  /// .try_into()?;
  ///
  /// let ctx_map = [("name", "Tom"), ("hi", "Hello!")]
  ///   .into_iter()
  ///   .map(|(k, v)| (k.into(), v.into()) )
  ///   .collect();
  ///
  /// let text = res.get_with_ctx_map("greeting", &ctx_map)?;
  /// assert_eq!(text, "Hello! Tom");
  ///
  /// # Ok::<(), glossa_dsl::error::ResolverError>(())
  /// ```
  pub fn get_with_ctx_map_buf(
    &self,
    var_name: &str,
    context_map: &crate::ContextMapBuf,
  ) -> ResolverResult<MiniStr> {
    let process = |ctx| self.try_get_template_and_process(var_name, ctx);

    match context_map.is_empty() {
      true => Context::Empty,
      _ => context_map.pipe(Context::MapBuf),
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

  #[test]
  #[ignore]
  #[cfg(feature = "std")]
  fn test_get_with_ctx_map() -> ResolverResult<()> {
    let res: Resolver = [
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

  #[test]
  fn test_get_with_btree_map() -> ResolverResult<()> {
    let res: Resolver = [
      ("greeting", "Good { time-period }! { $name }"),
      (
        "time-period",
        "$period ->
          [morning] Morning
          *[other] {$period}",
      ),
    ]
    .try_into()?;

    let ctx_map = [("name", "Tom"), ("period", "morning")]
      .into_iter()
      .map(|(k, v)| (k.into(), v.into()))
      .collect();

    let text = res.get_with_ctx_btree_map("greeting", &ctx_map)?;
    assert_eq!(text, "Good Morning! Tom");
    Ok(())
  }
}
