use std::{
  ffi::OsStr,
  io::{self, Write},
  process::{Child, Command, Stdio},
};

use getset::{Getters, Setters, WithSetters};
use tap::Pipe;

use crate::{bool_ext::BoolExt, os_cmd::DecodedText};

fn invalid_input_err(msg: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, msg)
}
fn empty_command_err() -> io::Error {
  invalid_input_err("empty command argv")
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
#[derive(Debug, Clone, PartialEq, Eq, WithSetters, Setters, Getters)]
#[getset(set = "pub", set_with = "pub", get = "pub with_prefix")]
pub struct CommandSpawner<'a, I>
where
  I: IntoIterator,
  I::Item: AsRef<OsStr>,
{
  stdin: StdioMode,
  stdout: StdioMode,
  stderr: StdioMode,
  command: Option<I>,
  stdin_data: Option<&'a [u8]>,
}

impl<'a, I> Default for CommandSpawner<'a, I>
where
  I: IntoIterator,
  I::Item: AsRef<OsStr>,
{
  /// default:
  ///
  /// ```ignore
  /// Self {
  ///     stdin: Inherit,
  ///     stdout: Inherit,
  ///     stderr: Inherit,
  ///     command: None,
  ///     stdin_data: None,
  /// }
  /// ```
  fn default() -> Self {
    use StdioMode::*;
    Self {
      stdin: Inherit,
      stdout: Inherit,
      stderr: Inherit,
      command: None,
      stdin_data: None,
    }
  }
}

impl<'a, I> CommandSpawner<'a, I>
where
  I: IntoIterator,
  I::Item: AsRef<OsStr>,
{
  #[inline]
  fn effective_stdin_mode(has_data: bool, stdin: StdioMode) -> StdioMode {
    use StdioMode::*;
    match (has_data, stdin) {
      (true, _) => Piped,
      // (false, Piped) => Inherit,
      _ => stdin,
    }
  }

  /// Spawn a child from an argv-like iterator:
  /// - first item: program
  /// - remaining items: args
  pub fn spawn(self) -> io::Result<Child> {
    let Self {
      command,
      stdin_data,
      stdin,
      stdout: stdout_mode,
      stderr: stderr_mode,
      ..
    } = self;

    let stdin_mode = Self::effective_stdin_mode(stdin_data.is_some(), stdin);

    command
      .ok_or_else(empty_command_err)?
      .into_iter()
      .pipe(|mut iter| {
        iter
          .next() // first arg
          .ok_or_else(empty_command_err) // Convert Option to Result
          .map(|prog| (prog, iter))
      })?
      .pipe(|(prog, iter)| {
        prog
          .pipe(Command::new)
          .args(iter)
          .stdin(stdin_mode)
          .stdout(stdout_mode)
          .stderr(stderr_mode)
          .spawn()
      })?
      .pipe(|child| Self::write_child_stdin(child, stdin_data))
  }

  pub fn write_child_stdin(
    mut child: Child,
    stdin_data: Option<&[u8]>,
  ) -> io::Result<Child> {
    if let Some(data) = stdin_data {
      child
        .stdin
        .as_mut()
        .ok_or_else(|| invalid_input_err("Failed to get stdin"))?
        .write_all(data)?
    }
    Ok(child)
  }

  #[inline]
  fn capture_output(
    self,
    cap_out: bool,
    cap_err: bool,
  ) -> io::Result<std::process::Output> {
    match (cap_out, cap_err) {
      (true, true) => self
        .with_stdout(StdioMode::Piped)
        .with_stderr(StdioMode::Piped),
      (true, false) => self.with_stdout(StdioMode::Piped),
      (false, true) => self.with_stderr(StdioMode::Piped),
      _ => self,
    }
    .spawn()?
    .wait_with_output()
  }

  pub fn capture_stdout(self) -> io::Result<DecodedText> {
    self
      .capture_output(true, false)?
      .stdout
      .pipe(DecodedText::from)
      .pipe(Ok)
  }

  pub fn capture_stderr(self) -> io::Result<DecodedText> {
    self
      .capture_output(false, true)?
      .stderr
      .pipe(DecodedText::from)
      .pipe(Ok)
  }

  pub fn capture_stdout_and_stderr(self) -> io::Result<[DecodedText; 2]> {
    self
      .capture_output(true, true)?
      .pipe(|o| [o.stdout, o.stderr])
      .map(DecodedText::from)
      .pipe(Ok)
  }
}
