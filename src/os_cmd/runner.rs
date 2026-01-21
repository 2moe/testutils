use alloc::borrow::Cow;
use std::{io, process::Command};

use getset::{Getters, Setters, WithSetters};
use tap::{Pipe, Tap};

use crate::{
  bool_ext::BoolExt,
  os_cmd::{CommandRepr, MiniStr, repr::TinyCmds},
};

/// Command runner with configurable preprocessing and execution strategies
///
/// - command: `[cmd, args...]`
/// - remove_comments: `remove //` (only for raw strings)
/// - inspect(debug) command: {eprint_cmd, log_dbg_cmd}
#[derive(Debug, Clone, WithSetters, Getters, Setters)]
#[getset(set = "pub", set_with = "pub", get = "pub with_prefix")]
pub struct Runner<'a> {
  /// Command representation (raw string or pre-split slices)
  command: CommandRepr<'a>,
  /// Flag to remove `//` comments from command strings
  remove_comments: bool,

  /// Directly print command to stderr before execution
  eprint_cmd: bool,
  /// Log command via `log::debug!()`,
  log_dbg_cmd: bool,
}

pub trait RunnableCommand<'a>: Sized
where
  Runner<'a>: From<Self>,
{
  fn run(self) -> io::Result<()> {
    Runner::from(self).run()
  }
}

impl<'a> RunnableCommand<'a> for Runner<'a> {}

impl Runner<'_> {
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
    let eprint_cmd = self.eprint_cmd;
    let log_dbg_cmd = self.log_dbg_cmd;

    // Phase 1: Command collection
    self
      .into_tinyvec()
      // Phase 2: Command inspection
      .tap(|v| match v {
        _ if eprint_cmd => eprintln!("{v:?}"), // Stderr output
        _ if log_dbg_cmd => log::debug!("{v:?}"), // Structured logging
        _ => {}
      })
      // Phase 3: OS command execution
      .iter()
      .pipe(run_os_cmd)
  }
}

impl<'a> Runner<'a> {
  /// - Raw(&str) => [collect_raw](Self::collect_raw) => command vec
  /// - Slice(Box<[&str]>) => `TinyVec<[Cow<&str>]>`
  /// - OwnedSlice(Box<[CompactString]>) => `TinyVec<[Cow<String>]>`
  pub fn into_tinyvec(self) -> TinyCmds<'a> {
    let Self {
      command,
      remove_comments,
      ..
    } = self;

    command.into_tinyvec(remove_comments)
  }
}
/// Core command execution logic
pub fn run_os_cmd(mut iter: core::slice::Iter<Cow<str>>) -> io::Result<()> {
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
    .then_ok_or_else(failed_to_run) // Convert bool to Result
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
  ///     command: CommandRepr::Raw("cargo"),
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
  #[cfg(feature = "print_ext")]
  fn show_default_runner() {
    Runner::default().pipe(|x| crate::dbg!(x));
  }
}
