use alloc::borrow::Cow;
use std::{io, process::Command};

use getset::{Getters, WithSetters};
use tap::{Pipe, Tap};

use crate::{
  os_cmd::{CommandRepr, MiniStr},
  tiny_container::TinyVec,
  traits::BoolExt,
};

/// Command runner with configurable preprocessing and execution strategies
#[derive(Debug, Clone, WithSetters, Getters)]
#[getset(set_with = "pub", get = "pub with_prefix")]
pub struct Runner<'a> {
  /// Command representation (raw string or pre-split slices)
  command: CommandRepr<'a>,
  /// Flag to remove `//` comments from command strings
  remove_comments: bool,
  /// Directly print command to stderr before execution
  eprint_cmd: bool,
  /// Log command via `log::debug!()`
  log_dbg_cmd: bool,
}

/// Preprocesses command string by removing comment lines
///
/// Why Cow:
///
/// - This involves an external condition check. When `remove_comments` is true,
///   this function is executed. When it is false, it directly returns the
///   original string (`&str`). To ensure both conditions return the same type,
///   this function wraps the returned string in `Cow`.
fn remove_comments_and_collect(s: &str) -> Cow<'_, str> {
  s.lines()
    .filter(|x| !x.trim().starts_with("//"))
    .collect::<String>()
    .pipe(Cow::from)
}

impl Runner<'_> {
  /// Parses raw command string into executable components
  ///
  /// Why TinyVec:
  /// - Stack-allocated for small commands (≤16 elements)
  /// - Fallback to heap for large commands automatically
  ///
  /// > size: `TinyVec<[Cow<'_, str>; 16]>` = 392
  pub fn collect_raw(
    raw: &str,
    remove_comments: bool,
  ) -> TinyVec<[Cow<'_, str>; 16]> {
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

  /// Executes command with configured preprocessing
  ///
  /// ## Example
  ///
  /// ```ignore
  /// use tap::Pipe;
  /// use testutils::{
  ///   get_pkg_name,
  ///   os_cmd::{Runner, presets::CargoDoc},
  /// };
  ///
  /// CargoDoc::default()
  ///   .with_pkg(get_pkg_name!())
  ///   .pipe(Runner::from)
  ///   .run()
  /// ```
  pub fn run(self) -> io::Result<()> {
    use CommandRepr::{OwnedSlice, Raw, Slice};

    // Phase 1: Command collection
    match self.command {
      Raw(raw) => Self::collect_raw(raw, self.remove_comments),
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
    // Phase 2: Command inspection
    .tap(|v| match v {
      _ if self.eprint_cmd => eprintln!("{v:?}"), // Stderr output
      _ if self.log_dbg_cmd => log::debug!("{v:?}"), // Structured logging
      _ => {}
    })
    // Phase 3: OS command execution
    .iter()
    .pipe(run_os_cmd)
  }
}

/// Core command execution logic
fn run_os_cmd(mut iter: core::slice::Iter<Cow<str>>) -> io::Result<()> {
  // Error helpers with lazy evaluation
  let err = |msg| io::Error::other(msg);
  let invalid_cmd = || "Invalid command".pipe(err);
  let failed_to_run = || "Failed to run OS command".pipe(err);

  iter
    .next()
    .map(AsRef::as_ref) // Dereference Cow transparently
    .ok_or_else(invalid_cmd)? // Convert Option to Result
    .pipe(Command::new) // Main command creation
    .args(iter.map(AsRef::as_ref)) // Remainder as arguments
    .status()? // Execute and get status
    .success() // Convert status to bool
    .ok_or_else(failed_to_run) // Convert bool to Result
}

/// Conversion trait implementation
impl<'a> From<CommandRepr<'a>> for Runner<'a> {
  fn from(command: CommandRepr<'a>) -> Self {
    Self {
      command,
      ..Default::default()
    }
  }
}

impl Default for Runner<'_> {
  /// Default:
  ///
  /// ```ignore
  /// Runner {
  ///     command: Raw("cargo"),
  ///     remove_comments: true,
  ///     eprint_cmd: true,
  ///     log_dbg_cmd: false,
  /// }
  /// ```
  ///
  /// Why these defaults:
  ///
  /// - remove_comments: true => Safer execution by default
  /// - eprint_cmd: true => Immediate visibility of executed command
  /// - log_dbg_cmd: false => Avoid duplicate logging unless requested
  fn default() -> Self {
    Self {
      command: CommandRepr::default(),
      remove_comments: true,
      eprint_cmd: true,
      log_dbg_cmd: false,
    }
  }
}

impl From<Vec<MiniStr>> for Runner<'_> {
  fn from(value: Vec<MiniStr>) -> Self {
    Self::default().with_command(value.into())
  }
}

impl From<Box<[MiniStr]>> for Runner<'_> {
  fn from(value: Box<[MiniStr]>) -> Self {
    Self::default().with_command(value.into())
  }
}
impl<'a> From<Box<[&'a str]>> for Runner<'a> {
  fn from(value: Box<[&'a str]>) -> Self {
    Self::default().with_command(value.into())
  }
}

impl<'a> From<&'a str> for Runner<'a> {
  fn from(value: &'a str) -> Self {
    Self::default().with_command(value.into())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[ignore]
  #[test]
  fn show_default_runner() {
    Runner::default().pipe(|x| dbg!(x));
  }
}
