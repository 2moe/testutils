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

/// `CommandSpawner` is a small builder that treats an iterator as an
/// `argv`-like sequence:
///
/// - The **first** item is the program to execute.
/// - The remaining items are passed as arguments, without going through a
///   shell.
///
/// In addition, you may provide `stdin_data`, which (if present) will be
/// written into the child's stdin after spawning. When `stdin_data` is set,
/// stdin is forced to `Piped` so the parent can write to it.
///
/// # Notes
///
/// - If you pipe **both** stdin (and write a lot of data) **and** pipe
///   stdout/stderr, be aware of potential deadlocks if the child writes enough
///   output to fill its pipe buffer while the parent is blocked writing stdin.
///   For large payloads, consider writing stdin from another thread while
///   concurrently reading output.
#[derive(Debug, Clone, PartialEq, Eq, WithSetters, Setters, Getters)]
#[getset(set = "pub", set_with = "pub", get = "pub with_prefix")]
pub struct CommandSpawner<'a, I>
where
  I: IntoIterator,
  I::Item: AsRef<OsStr>,
{
  /// child's stdin mode.
  stdin: StdioMode,
  /// child's stdout mode.
  stdout: StdioMode,
  /// child's stderr mode.
  stderr: StdioMode,

  /// An argv-like iterator where the first item is the program.
  ///
  /// If this is `None`, `spawn()` fails with an "empty command" error.
  command: Option<I>,

  /// Optional bytes to write into the child's stdin after spawning.
  ///
  /// When set, stdin will be forced to `Piped` so `write_all` can succeed.
  stdin_data: Option<&'a [u8]>,
}

impl<'a, I> Default for CommandSpawner<'a, I>
where
  I: IntoIterator,
  I::Item: AsRef<OsStr>,
{
  /// Creates a spawner with "inherit everything" stdio, and no command/data.
  ///
  /// ```ignore
  /// Self {
  ///   stdin:  Inherit,
  ///   stdout: Inherit,
  ///   stderr: Inherit,
  ///   command: None,
  ///   stdin_data: None,
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
  /// Computes the effective stdin mode.
  ///
  /// If `stdin_data` is present, stdin must be `Piped` so we can write to it.
  /// Otherwise, preserve the configured stdin mode as-is.
  ///
  /// This function is kept small and pure to stay friendly to a pipeline/FP
  /// style.
  #[inline]
  fn effective_stdin_mode(has_data: bool, stdin: StdioMode) -> StdioMode {
    use StdioMode::*;
    match (has_data, stdin) {
      (true, _) => Piped,
      // (false, Piped) => Inherit,
      _ => stdin,
    }
  }

  /// Spawns a child process from an argv-like iterator.
  ///
  /// The iterator is interpreted as:
  ///
  /// - first item: program
  /// - remaining items: args
  ///
  /// # Errors
  ///
  /// - Returns an error if `command` is `None`.
  /// - Returns an error if the iterator is empty (no program).
  /// - Propagates any I/O error from `Command::spawn`.
  /// - If `stdin_data` is set, returns an error if `stdin` is not available
  ///   (e.g., misconfigured to not be piped).
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
        // Split into (program, remaining args).
        iter
          .next()
          .ok_or_else(empty_command_err)
          .map(|prog| (prog, iter))
      })?
      .pipe(|(prog, iter)| {
        // Build and spawn the process without going through a shell.
        prog
          .pipe(Command::new)
          .args(iter)
          .stdin(stdin_mode)
          .stdout(stdout_mode)
          .stderr(stderr_mode)
          .spawn()
      })?
      // Optionally write stdin data, then return the (possibly modified) child.
      .pipe(|child| Self::write_child_stdin(child, stdin_data))
  }

  /// Writes `stdin_data` to the child's stdin (if present) and return the
  /// child.
  ///
  /// # Errors
  ///
  /// Returns an error if `stdin_data` is `Some(_)` but the child's stdin handle
  /// is not available (typically because stdin was not piped).
  pub fn write_child_stdin(
    mut child: Child,
    stdin_data: Option<&[u8]>,
  ) -> io::Result<Child> {
    if let Some(data) = stdin_data {
      child
        .stdin
        .as_mut()
        .ok_or_else(|| invalid_input_err("Failed to access child's stdin."))?
        .write_all(data)?;
    }
    Ok(child)
  }

  /// Spawns the process and capture output according to the requested streams.
  ///
  /// - When `cap_out` is true, stdout is forced to `Piped`.
  /// - When `cap_err` is true, stderr is forced to `Piped`.
  ///
  /// This returns the raw `std::process::Output` (bytes for stdout/stderr).
  /// Higher-level helpers (`capture_stdout`, `capture_stderr`,
  /// `capture_stdout_and_stderr`) decode those bytes into `DecodedText`.
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

  /// Captures stdout as decoded text.
  ///
  /// This forces stdout to `Piped`, spawns the child, waits for completion,
  /// and decodes `output.stdout` into `DecodedText`.
  pub fn capture_stdout(self) -> io::Result<DecodedText> {
    self
      .capture_output(true, false)?
      .stdout
      .pipe(DecodedText::from)
      .pipe(Ok)
  }

  /// Captures stderr as decoded text.
  ///
  /// This forces stderr to `Piped`, spawns the child, waits for completion,
  /// and decodes `output.stderr` into `DecodedText`.
  pub fn capture_stderr(self) -> io::Result<DecodedText> {
    self
      .capture_output(false, true)?
      .stderr
      .pipe(DecodedText::from)
      .pipe(Ok)
  }

  /// Captures both stdout and stderr as decoded text.
  ///
  /// This forces both streams to `Piped`, waits for completion,
  /// and returns `[stdout, stderr]` in that order.
  pub fn capture_stdout_and_stderr(self) -> io::Result<[DecodedText; 2]> {
    self
      .capture_output(true, true)?
      .pipe(|o| [o.stdout, o.stderr])
      .map(DecodedText::from)
      .pipe(Ok)
  }
}
