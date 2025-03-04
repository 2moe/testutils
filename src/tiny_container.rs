use alloc::{borrow::ToOwned, boxed::Box};
use core::{fmt, fmt::Write};

use tinyvec_string::TinyString;
pub use tinyvec_string::{
  bytearray::ByteArray,
  tinyvec::{Array, TinyVec},
};
/// TinyString
///
/// ## Example
///
/// ```
/// use testutils::tiny_container::TString;
///
/// let s: TString<5> = "Hello".into();
/// assert_eq!(s, "Hello");
/// ```
pub type TString<const N: usize> = TinyString<[u8; N]>;

pub trait Formattable<const N: usize>: Write {
  fn format(fmt_args: fmt::Arguments) -> Result<Self, fmt::Error>
  where
    Self: core::marker::Sized;
}

impl<const N: usize> Formattable<N> for TString<N> {
  /// Formats a new `TString` with the generic parameter `const N` (representing
  /// the inline byte size). If the capacity is exceeded, it will overflow to
  /// the heap.
  ///
  /// ## Example
  ///
  /// ```
  /// use tap::Pipe;
  /// use testutils::tiny_container;
  /// use testutils::tiny_container::TString;
  /// use crate::testutils::tiny_container::Formattable;
  ///
  /// let s = format_args!("Hello, {name}!", name = "World")
  ///   .pipe(TString::<16>::format);
  /// assert_eq!(s.as_deref(), Ok("Hello, World!"));
  /// // assert_eq!(core::mem::size_of_val(&s), 32);
  /// assert_eq!(s.map(|x| x.is_inline()), Ok(true))
  /// ```
  fn format(fmt_args: fmt::Arguments) -> Result<Self, fmt::Error> {
    let mut output = TString::new();
    output.write_fmt(fmt_args)?;
    Ok(output)
  }
}

pub trait IntoBoxedStr {
  fn into_boxed_str(self) -> Box<str>;
  fn into_string(self) -> String;
}
impl<A: ByteArray> IntoBoxedStr for TinyString<A> {
  /// Converts a TinyString into a `Box<str>`.
  ///
  /// ## Example
  ///
  /// ```
  /// use testutils::tiny_container::TString;
  /// use crate::testutils::tiny_container::IntoBoxedStr;
  ///
  /// let a = TString::<1>::from("a").into_boxed_str();
  /// assert!(core::any::type_name_of_val(&a).ends_with("Box<str>"));
  /// ```
  fn into_boxed_str(self) -> Box<str> {
    self
      .into_string()
      .into_boxed_str()
  }

  fn into_string(self) -> String {
    use TinyString::*;
    match self {
      Inline(arr_str) => arr_str.to_owned(),
      Heap(heap_string) => heap_string,
    }
  }
}

#[cfg(test)]
mod tests {

  use tap::Pipe;

  use super::*;

  #[ignore]
  #[test]
  fn test_tiny_container() {
    let s =
      format_args!("Hello, {name}!", name = "World").pipe(TString::<14>::format);

    assert_eq!(s.as_deref(), Ok("Hello, World!"));
    assert_eq!(s.map(|x| x.is_inline()), Ok(true));
  }

  #[test]
  fn test_tinystr_into_boxed_str() {
    let a = TString::<1>::from("a").into_boxed_str();
    assert!(core::any::type_name_of_val(&a).ends_with("Box<str>"));
  }

  #[test]
  fn test_tstring() {
    let s: TString<5> = "Hello".into();
    assert_eq!(s, "Hello");
  }
}
