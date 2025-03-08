use tap::Tap;
use tinyvec::TinyVec;

// use super::{ResolverResult, TemplateResolver};
use crate::{
  MiniStr,
  error::{ResolverError, ResolverResult},
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
      .tap_mut(|x| x.sort_unstable_by_key(|(k, _)| *k));

    self.process_template(template, &sorted_context)
  }

  #[cfg(feature = "std")]
  pub(crate) fn get_value_by_key(&self, key: &str) -> Option<&Template> {
    self.0.get(key)
  }

  #[cfg(not(feature = "std"))]
  pub(crate) fn get_value_by_key(&self, key: &str) -> Option<&Template> {
    let vec = &self.0;

    vec
      .binary_search_by_key(&key, |(k, _)| k)
      .ok()
      .and_then(|idx| vec.get(idx))
      .map(|x| &x.1)
  }
}
