use crate::MiniStr;

/// Template segment variants
///
/// ## Optimization
///
/// - Text variant stores content directly in MiniStr
/// - Variable uses efficient enum discriminants
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TemplatePart {
  Text(MiniStr),
  Variable(VariableRef),
}

impl Default for TemplatePart {
  #[inline]
  fn default() -> Self {
    const { Self::Text(MiniStr::const_new("")) }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VariableRef {
  Variable(MiniStr),
  Parameter(MiniStr),
}
