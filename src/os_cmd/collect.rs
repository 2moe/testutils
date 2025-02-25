use crate::os_cmd::{MiniStr, SmallString};

/// This function takes an iterator of items that can be converted into
/// `SmallString` and collects them into a `Box<[SmallString]>`.
///
/// ## Example
///
/// ```
/// use tap::Pipe;
/// use testutils::os_cmd::collect::collect_to_smallstr_slice;
///
/// let slice = ["cargo", "+nightly", "fmt"]
///   .into_iter()
///   .pipe(collect_to_smallstr_slice);
///
/// assert_eq!(slice.len(), 3);
/// assert_eq!(slice[0], "cargo");
/// assert_eq!(slice[1], "+nightly");
/// assert_eq!(slice[2], "fmt");
/// ```
pub fn collect_to_smallstr_slice<I>(iter: I) -> Box<[SmallString]>
where
  I: Iterator,
  I::Item: Into<SmallString>,
{
  iter.map(Into::into).collect()
}

/// iter => Box<[MiniStr]>
///
/// ## Example
///
/// ```
/// use tap::Pipe;
/// use testutils::os_cmd::collect::collect_to_ministr_slice;
///
/// let _slice = ["cargo", "+nightly", "fmt"]
///   .into_iter()
///   .pipe(collect_to_ministr_slice);
pub fn collect_to_ministr_slice<I>(iter: I) -> Box<[MiniStr]>
where
  I: Iterator,
  I::Item: Into<MiniStr>,
{
  iter.map(Into::into).collect()
}

#[cfg(test)]
mod tests {
  use tap::Pipe;

  use super::*;

  #[ignore]
  #[test]
  fn test_collect_owned_slice() {
    let slice = ["cargo", "+nightly", "fmt"]
      .into_iter()
      .pipe(collect_to_smallstr_slice);

    assert_eq!(slice.len(), 3);
    assert_eq!(slice[0], "cargo");
    assert_eq!(slice[1], "+nightly");
    assert_eq!(slice[2], "fmt");
  }
}
