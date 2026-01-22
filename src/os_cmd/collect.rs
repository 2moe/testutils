use crate::os_cmd::MiniStr;

/// iter => `Box<[MiniStr]>`
///
/// ## Example
///
/// ```
/// use tap::Pipe;
/// use testutils::os_cmd::collect_boxed_ministr_slice;
///
/// let slice = ["cargo", "+nightly", "fmt"].pipe(collect_boxed_ministr_slice);
///
/// assert_eq!(slice.len(), 3);
/// assert_eq!(slice[0], "cargo");
/// assert_eq!(slice[1], "+nightly");
/// assert_eq!(slice[2], "fmt");
/// ```
pub fn collect_boxed_ministr_slice<I>(iter: I) -> Box<[MiniStr]>
where
  I: IntoIterator,
  I::Item: Into<MiniStr>,
{
  iter
    .into_iter()
    .map(Into::into)
    .collect()
}

#[cfg(test)]
mod tests {
  use tap::Pipe;

  use super::*;

  #[ignore]
  #[test]
  fn test_collect_owned_slice() {
    let slice = ["cargo", "+nightly", "fmt"].pipe(collect_boxed_ministr_slice);

    assert_eq!(slice.len(), 3);
    assert_eq!(slice[0], "cargo");
    assert_eq!(slice[1], "+nightly");
    assert_eq!(slice[2], "fmt");
  }
}
