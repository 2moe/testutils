use std::io;

use getset::{CopyGetters, Getters, Setters, WithSetters};
use tap::{Pipe, Tap};

use crate::os_cmd::{
  CommandRepr, MiniStr, cow_str_into_cow_osstr, process::run_os_cmd, repr::TinyCmds,
};

/// Command runner with configurable preprocessing and execution strategies
///
/// - command: `[cmd, args...]`
/// - remove_comments: `remove //` (only for raw string, i.e., self.command ==
///   CommandRepr::Raw)
/// - inspect_mode: Emit the command via eprintln! or log::debug!
#[derive(Debug, Clone, WithSetters, Getters, Setters, CopyGetters)]
#[getset(set = "pub", set_with = "pub", get = "pub with_prefix")]
pub struct Runner<'a> {
  /// Command representation (raw string or pre-split slices)
  pub command: CommandRepr<'a>,
  /// Whether to strip `//`-style line comments from raw command strings.
  remove_comments: bool,

  /// Controls how (and whether) the command is surfaced for
  /// debugging/inspection.
  inspect_mode: RunnerInspection,
}

#[derive(Debug, Clone, Default, Copy)]
pub enum RunnerInspection {
  /// Write the command to stderr immediately before execution.
  #[default]
  Stderr,

  /// Emit the command via `log::debug!()` prior to execution.
  LogDebug,

  /// Do not emit the command.
  None,
}

pub trait RunnableCommand<'a>: Sized
where
  Runner<'a>: From<Self>,
{
  /// Executes command with configured preprocessing
  ///
  /// ## Example
  ///
  /// ```ignore
  /// use testutils::{get_pkg_name, os_cmd::presets::CargoDoc};
  ///
  /// CargoDoc::default()
  ///   .with_pkg(get_pkg_name!())
  ///   .run()
  /// ```
  fn run(self) -> io::Result<()> {
    Runner::from(self).run_command()
  }
}

impl<'a> RunnableCommand<'a> for Runner<'a> {}

impl Runner<'_> {
  /// see also: [RunnableCommand::run()]
  pub fn run_command(self) -> io::Result<()> {
    use RunnerInspection::{LogDebug, Stderr};
    let Self { inspect_mode, .. } = self;

    // Phase 1: Command collection
    self
      .into_tinyvec()
      // Phase 2: Command inspection
      .tap(|v| match inspect_mode {
        Stderr => eprintln!("{v:?}"),
        LogDebug => log::debug!("{v:?}"),
        _ => {}
      })
      // Phase 3: OS command execution
      .into_iter()
      .map(cow_str_into_cow_osstr)
      .pipe(run_os_cmd)
  }
}

impl<'a> Runner<'a> {
  /// - Raw(&str) => [collect_raw](super::collect_raw) => command vec
  /// - Slice(Box<[&str]>) => `TinyVec<[Cow<&str>]>`
  /// - OwnedSlice(Box<[compact_str::CompactString]>) =>
  ///   `TinyVec<[Cow<String>]>`
  pub fn into_tinyvec(self) -> TinyCmds<'a> {
    let Self {
      command,
      remove_comments,
      ..
    } = self;

    command.into_tinyvec(remove_comments)
  }
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
  ///     inspect_mode: RunnerInspection::Stderr,
  /// }
  /// ```
  fn default() -> Self {
    Self {
      command: CommandRepr::default(),
      remove_comments: true,
      inspect_mode: RunnerInspection::default(),
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
