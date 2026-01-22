use getset::{Getters, WithSetters};
use tap::Pipe;

use crate::os_cmd::MiniStr;

/// Decoded child-process output text, supporting both lossless and lossy UTF-8.
///
/// - `lossy`: When `true`, the output was decoded using
///   `String::from_utf8_lossy`, meaning the original byte stream contained
///   invalid UTF-8 sequences.
/// - `data`: The final string exposed to the caller.
///
/// ## Example
///
/// ```
/// use testutils::os_cmd::DecodedText;
///
/// let stdout = vec![108, 111, 115, 115, 108, 101, 115, 115, 10];
/// let output = DecodedText::from(stdout);
/// assert_eq!(output.lossy, false);
/// assert_eq!(output.data(), "lossless\n");
/// ```
#[derive(Debug, Clone, WithSetters, Getters, Default)]
#[getset(set_with = "pub")]
pub struct DecodedText {
  pub lossy: bool,
  #[getset(get = "pub")]
  data: MiniStr,
}

impl core::ops::Deref for DecodedText {
  type Target = MiniStr;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl core::fmt::Display for DecodedText {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(&self.data)
  }
}

// Converts any byte-like input into `DecodedText`.
// - If the bytes are valid UTF-8, keep it lossless.
// - Otherwise, decode with replacement (lossy) and mark `lossy = true`.
impl<B> From<B> for DecodedText
where
  B: AsRef<[u8]>,
{
  fn from(value: B) -> Self {
    let slice = value.as_ref();
    match MiniStr::from_utf8(slice) {
      Ok(s) => Self::new_lossless(s),
      // Fallback: replace invalid UTF-8 with U+FFFD and mark lossy.
      _ => slice
        .pipe(MiniStr::from_utf8_lossy)
        .pipe(Self::new_lossy),
    }
  }
}

impl DecodedText {
  /// Creates a decoded text value that came from valid UTF-8 (no replacement).
  pub fn new_lossless(data: MiniStr) -> Self {
    Self { lossy: false, data }
  }

  /// Creates a decoded text value where invalid UTF-8 may have been replaced.
  pub fn new_lossy(data: MiniStr) -> Self {
    Self { lossy: true, data }
  }

  /// Consumes self and return the underlying buffer data.
  pub fn into_compact_string(self) -> MiniStr {
    self.data
  }

  /// Alias for `into_compact_string`.
  pub fn take_data(self) -> MiniStr {
    self.data
  }

  /// Decodes from a byte vector (or anything that can become one).
  ///
  /// Small buffers go through the generic `From<[u8]>` path.
  /// Larger buffers avoid extra copies by decoding into an owned `String`
  /// first.
  pub fn from_vec<V: Into<Vec<u8>>>(v: V) -> Self {
    use std::borrow::Cow;

    let value = v.into();

    // Inline threshold for small strings (tuned by pointer width).
    const INLINE: usize = match usize::BITS {
      32 => 12,
      _ => 24,
    };

    if value.len() <= INLINE {
      return value.into();
    }

    // Converts an owned `String` into the compact string storage.
    let into_data = |s| MiniStr::from_string_buffer(s);

    match String::from_utf8_lossy(&value) {
      // Invalid UTF-8 was replaced, so this is lossy.
      Cow::Owned(s) => s
        .pipe(into_data)
        .pipe(Self::new_lossy),

      // Valid UTF-8: reuse the original bytes without re-checking.
      // SAFETY: `from_utf8_lossy` returned `Borrowed`, so `value` is valid UTF-8.
      _ => unsafe { String::from_utf8_unchecked(value) }
        .pipe(into_data)
        .pipe(Self::new_lossless),
    }
  }
}
