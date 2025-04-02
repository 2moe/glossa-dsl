use crate::{
  Resolver,
  resolver::{AST, OrderedAST},
};

impl Resolver {
  /// Converts the resolver into a BTreeMap.
  ///
  /// An ordered BTreeMap is useful when you need to serialize the
  /// Resolver to a configuration file or a binary file.
  ///
  /// ## Example
  ///
  /// ```
  /// use glossa_dsl::Resolver;
  ///
  /// let resolver = Resolver::default();
  /// let _map = resolver.into_btree_map();
  /// ```
  pub fn into_btree_map(self) -> OrderedAST {
    self.into()
  }

  /// Takes ownership of the Self and returns the inner data
  /// (template AST)
  ///
  /// ## Example
  ///
  /// ```
  /// use glossa_dsl::Resolver;
  ///
  /// let resolver = Resolver::default();
  /// let _inner_data = resolver.into_inner();
  /// ```
  pub fn into_inner(self) -> AST {
    self.0
  }
}

/// Converts the resolver into an ordered abstract syntax tree (AST)
/// representation.
///
/// ## Feature-dependent Behavior
///
/// - ​**With `std` feature**: Converts internal storage to a
///   [`alloc::collections::BTreeMap`]-backed ordered AST through iterative
///   collection. This guarantees deterministic ordering.
/// - ​**Without `std` feature**: Directly returns the pre-ordered AST structure
///   without conversion, optimized for no_std environments.
impl From<Resolver> for OrderedAST {
  fn from(value: Resolver) -> Self {
    #[cfg(feature = "std")]
    {
      value
        .into_inner()
        .into_iter()
        .collect()
    }

    #[cfg(not(feature = "std"))]
    {
      value.into_inner()
    }
  }
}
