use std::{
  borrow::Cow,
  ffi::{OsStr, OsString},
};

use tap::Pipe;

use crate::os_cmd::{CommandSpawner, Runner, cow_str_into_cow_osstr};

pub type ArgvItem<'a> = Cow<'a, OsStr>;

impl<'a, I> From<Runner<'a>> for CommandSpawner<'a, I>
where
  I: FromIterator<ArgvItem<'a>>,
  I: IntoIterator<Item = ArgvItem<'a>>,
{
  /// Runner -> iterator of Cow<str> -> Argv<...> -> CommandSpawner
  fn from(value: Runner<'a>) -> Self {
    value
      .into_tinyvec()
      .into_iter()
      .pipe(Argv)
      .into()
  }
}

/// Newtype wrapper to avoid overlapping `From<T>` impls.
#[derive(Debug, Clone, Copy)]
pub struct Argv<T>(pub T);

/// Convert a value into an argv item (`Cow<'a, OsStr>`).
///
/// We use a custom trait because:
/// - `ArgvItem<'a>` is a type alias, so we cannot impl traits for it directly.
/// - `Into<Cow<'a, OsStr>>` is not available for some common types like `&str`.
pub trait IntoArgvItem<'a> {
  fn into_argv_item(self) -> ArgvItem<'a>;
}

// --- already-in-OS-string forms ---

impl<'a> IntoArgvItem<'a> for ArgvItem<'a> {
  #[inline]
  fn into_argv_item(self) -> ArgvItem<'a> {
    self
  }
}

impl<'a> IntoArgvItem<'a> for &'a OsStr {
  #[inline]
  fn into_argv_item(self) -> ArgvItem<'a> {
    Cow::Borrowed(self)
  }
}

impl<'a> IntoArgvItem<'a> for OsString {
  #[inline]
  fn into_argv_item(self) -> ArgvItem<'a> {
    Cow::Owned(self)
  }
}

// --- Rust strings (UTF-8) ---

impl<'a> IntoArgvItem<'a> for &'a str {
  #[inline]
  fn into_argv_item(self) -> ArgvItem<'a> {
    Cow::Borrowed(OsStr::new(self))
  }
}

impl<'a> IntoArgvItem<'a> for String {
  #[inline]
  fn into_argv_item(self) -> ArgvItem<'a> {
    Cow::Owned(OsString::from(self))
  }
}

impl<'a> IntoArgvItem<'a> for Cow<'a, str> {
  #[inline]
  fn into_argv_item(self) -> ArgvItem<'a> {
    cow_str_into_cow_osstr(self)
  }
}

impl<'a, I, T> From<Argv<T>> for CommandSpawner<'a, I>
where
  T: IntoIterator,
  T::Item: IntoArgvItem<'a>,
  I: FromIterator<ArgvItem<'a>>,
  I: IntoIterator<Item = ArgvItem<'a>>,
{
  fn from(value: Argv<T>) -> Self {
    value
      .0
      .into_iter()
      .map(IntoArgvItem::into_argv_item)
      .collect::<I>()
      .pipe(|argv| CommandSpawner::default().with_command(Some(argv)))
  }
}
