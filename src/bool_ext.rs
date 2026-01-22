//! Extension trait for types that can be converted into a bool.
pub trait BoolExt {
  /// Converts the bool value into a `Result<(), E>`.
  ///
  /// If the value is `true`, it returns `Ok(())`. If the value is `false`, it
  /// returns `Err(err())`.
  ///
  /// # Examples
  ///
  /// ## Simple
  ///
  /// ```
  /// use testutils::bool_ext::BoolExt;
  ///
  /// let value = true;
  /// let res: Result<(), &str> = value.then_ok_or_else(|| "error");
  /// assert_eq!(res, Ok(()));
  ///
  /// let value = false;
  /// assert_eq!(value.then_ok_or_else(|| "error"), Err("error"));
  /// ```
  ///
  /// ## A more complex example
  ///
  /// ```ignore
  /// use testutils::traits::Pipe;
  /// use testutils::get_pkg_name;
  /// use testutils::testutils::bool_ext::BoolExt;
  /// use std::io;
  ///
  /// let err = || "Failed to run `cargo rustdoc` command".pipe(io::Error::other);
  ///
  /// get_pkg_name!()
  ///  .pipe(build_rsdoc)? // ExitStatus
  ///  .success() // bool
  ///  .then_ok_or_else(err) // io::Result<()>
  /// ```
  fn then_ok_or_else<E>(self, err_fn: impl FnOnce() -> E) -> Result<(), E>
  where
    Self: Into<bool>,
  {
    if self.into() { Ok(()) } else { Err(err_fn()) }
  }

  fn then_ok_or<E>(self, err: E) -> Result<(), E>
  where
    Self: Into<bool>,
  {
    if self.into() { Ok(()) } else { Err(err) }
  }
}

impl BoolExt for bool {}

#[cfg(test)]
mod tests {
  use tap::TapOptional;

  use super::*;

  #[ignore]
  #[test]
  fn test_tap_opt() {
    let opt = Some(42);
    opt.tap_some(|&x| assert_eq!(x, 42));
    opt.map(|x| assert_eq!(x, 42));
  }

  #[ignore]
  #[test]
  fn test_bool_ok_or_else() {
    let value = true;
    let res: Result<(), &str> = value.then_ok_or_else(|| "error");
    assert_eq!(res, Ok(()));

    let value = false;
    assert_eq!(value.then_ok_or_else(|| "error"), Err("error"));
  }

  #[cfg(feature = "std")]
  #[ignore]
  #[test]
  fn test_bool_ok_or_std_io_error() {
    // use std::io;
  }
}
