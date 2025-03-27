use thiserror::Error;

pub type ResolverResult<T> = Result<T, ResolverError>;

use crate::MiniStr;

/// Error hierarchy for template resolution
///
/// ## Error Handling Strategy
/// - Detailed error variants for precise diagnostics
/// - Uses thiserror for idiomatic error handling
/// - Compact error strings for low-overhead reporting
#[derive(Debug, Error)]
pub enum ResolverError {
  #[error("Undefined variable: {0}")]
  UndefinedVariable(MiniStr),
  //
  #[error("Missing paramter: {0}")]
  MissingParameter(MiniStr),
  //
  #[error("No default branch: {0}")]
  NoDefaultBranch(MiniStr),
  //
  #[error("Parse error: {0}")]
  ParseError(MiniStr),
  //
  #[cfg(feature = "std")]
  #[error("I/O error: {0}")]
  IoError(#[from] std::io::Error),
  //
  #[cfg(feature = "bincode")]
  #[error("Bincode serialization error: {0}")]
  EncodeBinError(#[from] bincode::error::EncodeError),
  //
  #[cfg(feature = "bincode")]
  #[error("Bincode deserialization error: {0}")]
  DecodeBinError(#[from] bincode::error::DecodeError),

  #[cfg(feature = "toml")]
  #[error("TOML deserialization error: {0}")]
  DecodeTomlError(#[from] toml::de::Error),

  #[cfg(feature = "toml")]
  #[error("TOML serialization error: {0}")]
  EncodeTomlError(#[from] toml::ser::Error), //
}
