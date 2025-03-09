use tap::Pipe;

use crate::{TemplateResolver, error::ResolverResult};

impl TemplateResolver {
  /// Decodes binary data into TemplateResolver using bincode's optimized
  /// deserialization.
  ///
  /// ## Input
  ///
  /// - `slice` - Binary input slice containing serialized TemplateResolver data
  ///
  /// ## Output
  ///
  /// - A tuple of (deserialized TemplateResolver, amount of bytes read) on
  ///   success
  /// - Error details if deserialization fails
  ///
  /// See also: [bincode::serde::decode_from_slice]
  pub fn decode_bin_from_slice(slice: &[u8]) -> ResolverResult<(Self, usize)> {
    let cfg = bincode::config::standard();
    bincode::serde::decode_from_slice(slice, cfg)?.pipe(Ok)
  }
}
