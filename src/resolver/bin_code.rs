use std::{
  fs::File,
  io::{BufReader, BufWriter},
  path::Path,
};

use bincode::serde::{decode_from_std_read, encode_into_std_write};
use tap::Pipe;

use crate::{
  TemplateResolver, error::ResolverResult, resolver::bin_code_nostd::bincode_std_cfg,
};

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
    let encode = |dst| encode_into_std_write(self, dst, bincode_std_cfg());

    dst_file
      .pipe(File::create)?
      .pipe(BufWriter::new) // Buffer writes for efficiency
      .pipe_ref_mut(encode)? // Stream encoding
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
    let decode = |src| decode_from_std_read::<Self, _, _>(src, bincode_std_cfg());

    src_file
      .pipe(File::open)? // Open with read-only access
      .pipe(BufReader::new) // Buffer reads for performance
      .pipe_ref_mut(decode)? // Stream decoding
      .pipe(Ok)
  }
}
