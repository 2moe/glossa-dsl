use crate::{
  TemplateResolver,
  error::{ResolverError, ResolverResult},
  parsers::parse_value_or_map_err,
  resolver::{AHashRawMap, TemplateAST},
};

impl TryFrom<AHashRawMap> for TemplateResolver {
  type Error = ResolverError;

  fn try_from(value: AHashRawMap) -> Result<Self, Self::Error> {
    Self::from_raw(value)
  }
}

impl TemplateResolver {
  /// Construct from HashMap (std only)
  ///
  /// ## Example
  ///
  /// ```
  ///   use tmpl_resolver::TemplateResolver;
  ///   use tmpl_resolver::resolver::AHashRawMap;
  ///
  ///   let map = [
  ///     (
  ///       "salutation",
  ///       "
  ///       $gender ->
  ///         [male] Mr.
  ///         *[female] Ms.",
  ///     ),
  ///     ("g", "Good"),
  ///     (
  ///       "time-period",
  ///       "$period ->
  ///         [morning] {g} Morning
  ///         [evening] {g} evening
  ///         *[other] {g} {$period}
  ///       ",
  ///     ),
  ///     ("greeting", "{ time-period }! { salutation }{ $name }"),
  ///   ]
  ///   .into_iter()
  ///   .map(|(k, v)| (k.into(), v.into()))
  ///   .collect::<AHashRawMap>();
  ///
  ///   let resolver = TemplateResolver::from_raw(map).expect("Failed to parse
  /// raw map");
  ///
  ///   let text = resolver.get_with_context(
  ///     "greeting",
  ///     &[
  ///       ("period", "evening"),
  ///       ("name", "Alice"),
  ///       ("gender", "unknown"),
  ///     ],
  ///   ).expect("Failed to get text");
  ///   assert_eq!(text, "Good evening! Ms.Alice");
  /// ```
  pub fn from_raw(raw: AHashRawMap) -> ResolverResult<Self> {
    use tap::Pipe;

    raw
      .into_iter()
      .map(|(key, value)| {
        parse_value_or_map_err(&key, &value) //
          .map(|tmpl| (key, tmpl))
      })
      // .tap_dbg(|x| println!("{:?}", x.size_hint()))
      .collect::<Result<TemplateAST, _>>()?
      .pipe(Self)
      .pipe(Ok)
  }
}
