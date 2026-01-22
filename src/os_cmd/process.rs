use std::{
  ffi::OsStr,
  io,
  process::{Child, Command, Stdio},
};

use getset::{Getters, Setters, WithSetters};
use tap::Pipe;

use crate::{bool_ext::BoolExt, os_cmd::DecodedText};

fn empty_command_err() -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, "empty command argv")
}

/// Runs an OS command without capturing stdout/stderr (inherits the parent's
/// stdio).
pub fn run_os_cmd<I>(into_iter: I) -> io::Result<()>
where
  I: IntoIterator,
  I::Item: AsRef<OsStr>,
{
  let mut iter = into_iter.into_iter();

  let program = iter
    .next()
    .ok_or_else(empty_command_err)?
    .as_ref()
    .to_os_string();

  let failed_to_run =
    || format!("Failed to run command: {program:?}").pipe(io::Error::other);

  Command::new(&program) // Main command creation
    .args(iter) // Remainder as arguments
    .status()? // Execute and get status
    .success() // Convert status to bool
    .then_ok_or_else(failed_to_run) // Convert bool to Result
}

/// How to wire a stdio stream for the child process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StdioMode {
  /// Inherit from current process.
  #[default]
  Inherit,
  /// Create a pipe between parent and child.
  Piped,
  /// Redirect to null device.
  Null,
}

impl From<StdioMode> for Stdio {
  fn from(val: StdioMode) -> Self {
    use StdioMode::*;
    match val {
      Inherit => Stdio::inherit(),
      Piped => Stdio::piped(),
      Null => Stdio::null(),
    }
  }
}

/// Spawn OS commands from argv-like iterators, with configurable stdio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, WithSetters, Setters, Getters)]
#[getset(set = "pub", set_with = "pub", get = "pub with_prefix")]
pub struct CommandSpawner<I>
where
  I: IntoIterator,
  I::Item: AsRef<OsStr>,
{
  stdin: StdioMode,
  stdout: StdioMode,
  stderr: StdioMode,
  command: Option<I>,
}

impl<I> Default for CommandSpawner<I>
where
  I: IntoIterator,
  I::Item: AsRef<OsStr>,
{
  /// default:
  ///
  /// ```ignore
  /// Self {
  ///     stdin: Inherit,
  ///     stdout: Piped,
  ///     stderr: Inherit,
  ///     command: None,
  /// }
  /// ```
  fn default() -> Self {
    use StdioMode::*;
    Self {
      stdin: Inherit,
      stdout: Piped,
      stderr: Inherit,
      command: None,
    }
  }
}

impl<I> CommandSpawner<I>
where
  I: IntoIterator,
  I::Item: AsRef<OsStr>,
{
  /// Spawn a child from an argv-like iterator:
  /// - first item: program
  /// - remaining items: args
  pub fn spawn(self) -> io::Result<Child> {
    let Self { command, .. } = self;
    let Some(command_iter) = command else {
      Err(empty_command_err())?
    };

    let mut iter = command_iter.into_iter();

    iter
      .next() // first arg
      .ok_or_else(empty_command_err)? // Convert Option to Result
      .pipe(Command::new) // Main command creation
      .args(iter)
      .stdin(self.stdin)
      .stdout(self.stdout)
      .stderr(self.stderr)
      .spawn()
  }

  pub fn capture_stdout(self) -> io::Result<DecodedText> {
    self
      .spawn()?
      .wait_with_output()?
      .stdout
      .pipe(DecodedText::from)
      .pipe(Ok)
  }
}
