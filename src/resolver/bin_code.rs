use std::{
  fs::File,
  io::{BufReader, BufWriter},
  path::Path,
};

use bincode::serde::{decode_from_std_read, encode_into_std_write};
use tap::Pipe;

use crate::{TemplateResolver, error::ResolverResult};

impl TemplateResolver {
  /// Serializes the resolver to bincode format and writes to a file
  ///
  /// ## Design Notes
  ///
  /// - Uses buffered writer for optimal large-file performance
  /// - Leverages bincode's compact binary representation
  /// - Preserves structure ordering through deterministic serialization
  ///
  /// ## Example
  ///
  /// ```no_run
  /// use tmpl_resolver::TemplateResolver;
  ///
  /// let resolver: TemplateResolver = [
  ///     ("h", "Hello"),
  ///     ("greeting", "{h} { $name }!")
  ///   ]
  ///   .try_into()
  ///   .expect("Invalid slice");
  ///
  /// let file = "tmp.bincode";
  ///
  /// resolver.encode_bin(file).expect("Failed to encode TemplateResolver to bincode file");
  /// ```
  pub fn encode_bin<P: AsRef<Path>>(&self, dst_file: P) -> ResolverResult<usize> {
    let cfg = bincode::config::standard();

    dst_file
      .pipe(File::create)?
      .pipe(BufWriter::new) // Buffer writes for efficiency
      .pipe_ref_mut(|dst| encode_into_std_write(self, dst, cfg))? // Stream encoding
      .pipe(Ok)
  }

  /// Deserializes a resolver from bincode-formatted file
  ///
  /// It Uses buffered reading for I/O efficiency.
  ///
  /// ## Example
  ///
  /// ```no_run
  /// use tmpl_resolver::TemplateResolver;
  ///
  /// let file = "tmp.bincode";
  ///
  /// TemplateResolver::decode_bin(file).expect("Failed to decode bincode file to TemplateResolver");
  /// ```
  pub fn decode_bin<P: AsRef<Path>>(src_file: P) -> ResolverResult<Self> {
    let cfg = bincode::config::standard();

    src_file
      .pipe(File::open)? // Open with read-only access
      .pipe(BufReader::new) // Buffer reads for performance
      .pipe_ref_mut(|src| decode_from_std_read::<Self, _, _>(src, cfg))? // Stream decoding
      .pipe(Ok)
  }
}
