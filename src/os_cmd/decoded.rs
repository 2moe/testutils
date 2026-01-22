use std::borrow::Cow;

use getset::{Getters, WithSetters};
use tap::Pipe;

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
  data: String,
}

impl From<Vec<u8>> for DecodedText {
  /// SAFETY: see also [String::from_utf8_lossy_owned]
  fn from(value: Vec<u8>) -> Self {
    match String::from_utf8_lossy(&value) {
      Cow::Owned(string) => DecodedText::new_lossy(string),
      _ => {
        unsafe { String::from_utf8_unchecked(value) }.pipe(DecodedText::new_lossless)
      }
    }
  }
}

impl DecodedText {
  pub fn new_lossless(data: String) -> Self {
    Self { lossy: false, data }
  }

  pub fn new_lossy(data: String) -> Self {
    Self { lossy: true, data }
  }
}
