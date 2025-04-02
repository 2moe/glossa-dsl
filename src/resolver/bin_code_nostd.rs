pub(crate) use bincode::config::standard as bincode_std_cfg;
use tap::Pipe;

use crate::{Resolver, error::ResolverResult};

impl Resolver {
  /// Decodes binary data into Resolver using bincode's optimized
  /// deserialization.
  ///
  /// ## Input
  ///
  /// - `slice` - Binary input slice containing serialized Resolver data
  ///
  /// ## Output
  ///
  /// - A tuple of (deserialized Resolver, amount of bytes read) on success
  /// - Error details if deserialization fails
  ///
  /// See also: [bincode::serde::decode_from_slice]
  pub fn decode_bin_from_slice(slice: &[u8]) -> ResolverResult<(Self, usize)> {
    bincode::serde::decode_from_slice(slice, bincode_std_cfg())?.pipe(Ok)
  }

  /// Encodes the Self(Resolver) into a binary format stored in a
  /// [`Vec<u8>`].
  ///
  /// ## Example
  ///
  /// ```
  /// use glossa_dsl::Resolver;
  ///
  /// let res: Resolver =
  ///   [("🐱", "喵 ฅ(°ω°ฅ)"), ("hello", "{🐱}")].try_into()?;
  ///
  /// let _data = res.encode_bin_to_vec()?;
  ///
  /// # Ok::<(), glossa_dsl::error::ResolverError>(())
  /// ```
  ///
  /// See also: [bincode::serde::encode_to_vec]
  pub fn encode_bin_to_vec(&self) -> ResolverResult<alloc::vec::Vec<u8>> {
    bincode::serde::encode_to_vec(self, bincode_std_cfg())?.pipe(Ok)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// - release: 1.542µs
  /// - debug: 7.375µs
  #[test]
  #[ignore]
  #[cfg(feature = "std")]
  fn bench_encode_bin() -> ResolverResult<()> {
    use testutils::simple_benchmark;

    // use glossa_dsl::Resolver;
    use crate::Resolver;
    let res: Resolver = [("🐱", "喵 ฅ(°ω°ฅ)"), ("hello", "{🐱}")].try_into()?;

    simple_benchmark(|| res.encode_bin_to_vec());

    let _data = res.encode_bin_to_vec()?;

    // println!("{}", data.capacity());
    // dbg!(data);

    Ok(())
  }
}
