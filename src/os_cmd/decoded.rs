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

impl<B> From<B> for DecodedText
where
  B: AsRef<[u8]>,
{
  fn from(value: B) -> Self {
    let slice = value.as_ref();
    match MiniStr::from_utf8(slice) {
      Ok(s) => Self::new_lossless(s),
      _ => slice
        .pipe(MiniStr::from_utf8_lossy)
        .pipe(Self::new_lossy),
    }
  }
}

impl DecodedText {
  pub fn new_lossless(data: MiniStr) -> Self {
    Self { lossy: false, data }
  }

  pub fn new_lossy(data: MiniStr) -> Self {
    Self { lossy: true, data }
  }

  /// Consumes the struct and returns the underlying `CompactString`.
  pub fn into_compact_string(self) -> MiniStr {
    self.data
  }

  /// Same as [Self::into_compact_string]
  pub fn take_data(self) -> MiniStr {
    self.data
  }

  fn contains_lossy_char(s: &str) -> bool {
    s.contains('\u{FFFD}')
  }

  pub fn from_vec<V: Into<Vec<u8>>>(v: V) -> Self {
    use std::borrow::Cow;

    let value = v.into();

    const INLINE: usize = match usize::BITS {
      32 => 12,
      _ => 24,
    };

    if value.len() <= INLINE {
      return value.into();
    }

    let into_data = |s| MiniStr::from_string_buffer(s);

    match String::from_utf8_lossy(&value) {
      Cow::Owned(s) => s
        .pipe(into_data)
        .pipe(Self::new_lossy),
      // SAFETY: see also [String::from_utf8_lossy_owned]
      _ => unsafe { String::from_utf8_unchecked(value) }
        .pipe(into_data)
        .pipe(Self::new_lossless),
    }
  }

  pub fn from_string<S: Into<String>>(s: S) -> Self {
    let value = s.into();
    let lossy = Self::contains_lossy_char(&value);
    let data = MiniStr::from_string_buffer(value);
    Self { lossy, data }
  }

  pub fn from_compact_string<S: Into<MiniStr>>(s: S) -> Self {
    let data = s.into();
    let lossy = Self::contains_lossy_char(&data);
    Self { lossy, data }
  }
}
