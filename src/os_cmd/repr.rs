use alloc::borrow::Cow;

use tap::Pipe;

use crate::os_cmd::{MiniStr, presets::CowStrVec};

pub(crate) type TinyCmds<'a> = CowStrVec<'a, 10>;

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

impl<'a> CommandRepr<'a> {
  /// - Raw(&str) => [collect_raw](Self::collect_raw) => command vec
  /// - Slice(Box<[&str]>) => `TinyVec<[Cow<&str>]>`
  /// - OwnedSlice(Box<[CompactString]>) => `TinyVec<[Cow<String>]>`
  pub fn into_tinyvec(self, remove_comments: bool) -> TinyCmds<'a> {
    use CommandRepr::{OwnedSlice, Raw, Slice};

    match self {
      Raw(raw) => collect_raw(raw, remove_comments),
      Slice(items) => items
        .into_iter()
        .map(Cow::from)
        .collect(),
      OwnedSlice(items) => items
        .into_iter()
        .map(|x| x.into_string())
        .map(Cow::from)
        .collect(),
    }
  }
}

/// Parses raw command string into executable components
///
/// Why TinyVec:
/// - Stack-allocated for small commands (≤10 elements)
/// - Fallback to heap for large commands automatically
///
/// > size: in x64 Linux, `TinyVec<[Cow<'_, str>; 10]>`: 248
pub fn collect_raw(raw: &str, remove_comments: bool) -> TinyCmds {
  raw
    .trim_ascii() // Trim ASCII whitespace efficiently (rust 1.80+)
    .pipe(|s| match remove_comments {
      true => remove_comments_and_collect(s),
      _ => s.into(), // Convert to Cow without cloning
    })
    .pipe_deref(shlex::Shlex::new) // Safe command line splitting
    .map(Cow::from)
    .collect()
}

/// Preprocesses command string by removing comment lines
///
/// Why Cow:
///
/// - This involves an external condition check. (Required by
///   [collect_raw](CommandRepr::collect_raw))
///
///   - When `remove_comments` is true, this function is executed.
///   - When it is false, it directly returns the original string (`&str`).
///   - To ensure both conditions return the same type, this function wraps the
///     returned string in `Cow`.
pub fn remove_comments_and_collect(s: &str) -> Cow<'_, str> {
  s.lines()
    // Since shell commands can contain `\"\n{space} \"`,
    // don't use `.map(|x| x.trim())` here.
    .filter(|x| {
      !x.trim_ascii_start()
        .starts_with("//")
    })
    .collect::<String>()
    .pipe(Cow::from)
}
