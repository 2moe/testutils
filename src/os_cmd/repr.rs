use crate::os_cmd::MiniStr;

/// Command Representation
///
/// - Raw: raw &str (e.g., "cargo +nightly fmt")
/// - Slice: `Box<[&str]>` (e.g., `vec!["cargo", "+nightly",
///   "fmt"].into_boxed_slice()`)
/// - OwnedSlice: `Box<[MiniStr]>` (e.g., `["cargo", "+nightly",
///   "fmt"].into_iter().pipe(collect_to_ministr_slice)`)
#[derive(Debug, Clone)]
pub enum CommandRepr<'a> {
  Raw(&'a str),
  Slice(Box<[&'a str]>),
  OwnedSlice(Box<[MiniStr]>),
}

impl Default for CommandRepr<'_> {
  /// Default: Raw("cargo")
  fn default() -> Self {
    Self::Raw("cargo")
  }
}

impl From<Box<[MiniStr]>> for CommandRepr<'_> {
  fn from(value: Box<[MiniStr]>) -> Self {
    Self::OwnedSlice(value)
  }
}

impl From<Vec<MiniStr>> for CommandRepr<'_> {
  fn from(value: Vec<MiniStr>) -> Self {
    Self::OwnedSlice(value.into())
  }
}

impl<'a> From<Box<[&'a str]>> for CommandRepr<'a> {
  fn from(value: Box<[&'a str]>) -> Self {
    Self::Slice(value)
  }
}
impl<'a> From<&'a str> for CommandRepr<'a> {
  fn from(value: &'a str) -> Self {
    Self::Raw(value)
  }
}
