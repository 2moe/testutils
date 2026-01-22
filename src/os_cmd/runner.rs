use std::io;

use getset::{CopyGetters, Getters, Setters, WithSetters};
use tap::{Pipe, Tap};

use crate::{
  bool_ext::BoolExt,
  os_cmd::{
    CommandRepr, CommandSpawner, DecodedText, cow_str_into_cow_osstr,
    process::{err_failed_to_run, run_os_cmd},
    repr::TinyCmds,
  },
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
  pub stdin_data: Option<&'a [u8]>,

  /// Whether to strip `//`-style line comments from raw command strings.
  pub(crate) remove_comments: bool,

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

  /// See also: [CommandSpawner::capture_stdout]
  fn capture_stdout(self) -> io::Result<DecodedText> {
    CommandSpawner::from(self).capture_stdout()
  }

  /// See also: [CommandSpawner::capture_stderr]
  fn capture_stderr(self) -> io::Result<DecodedText> {
    CommandSpawner::from(self).capture_stderr()
  }

  /// See also: [CommandSpawner::capture_stdout_and_stderr]
  fn capture_stdout_and_stderr(self) -> io::Result<[DecodedText; 2]> {
    CommandSpawner::from(self).capture_stdout_and_stderr()
  }
}

impl<'a> RunnableCommand<'a> for Runner<'a> {}

impl Runner<'_> {
  /// see also: [RunnableCommand::run()]
  pub fn run_command(self) -> io::Result<()> {
    use RunnerInspection::{LogDebug, Stderr};
    let Self { inspect_mode, .. } = self;

    if self.get_stdin_data().is_some() {
      return self
        .pipe(CommandSpawner::from)
        .tap(|x| match inspect_mode {
          LogDebug => log::debug!("{x:#?}"),
          Stderr => eprintln!("{x:#?}"),
          _ => {}
        })
        .spawn()?
        .wait()?
        .success()
        .then_ok_or_else(|| err_failed_to_run(None));
    }

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
      stdin_data: None,
    }
  }
}

impl<'a, T> From<T> for Runner<'a>
where
  T: Into<CommandRepr<'a>>,
{
  fn from(value: T) -> Self {
    Self {
      command: value.into(),
      ..Default::default()
    }
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
