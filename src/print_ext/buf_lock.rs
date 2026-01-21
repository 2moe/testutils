/*!
Buffered + locked stdio helpers.

This module provides `BufWriter`-wrapped `stdout`/`stderr` handles that are
also *locked*, which is a common pattern for high-throughput output:

- `StdoutLock`/`StderrLock` avoids re-locking the global stdio handle on
  every write.
- `BufWriter` batches many small writes into fewer syscalls.

## Notes

- The returned writers are **buffered**: output may not appear immediately.
  Call [`.flush()`](std::io::Write::flush) when you need timely output
  (e.g., prompts/progress).
- The underlying lock is held for as long as the writer value is alive. Keep
  the lifetime short if other threads also write to stdio.
*/
// ===========================
use std::io::{self, BufWriter};

/// A buffered, locked handle to standard output.
///
/// Uses `StdoutLock<'static>` because `stdout()` is backed by a global,
/// process-wide handle; the lock guard itself is still released on `Drop`.
pub type BufStdout = BufWriter<io::StdoutLock<'static>>;

/// A buffered, locked handle to standard error.
///
/// Like `BufStdout`, this holds a lock guard for the lifetime of the value
/// and buffers writes until the buffer is flushed/dropped.
pub type BufStderr = BufWriter<io::StderrLock<'static>>;

/// Creates a buffered, locked `stdout` writer.
///
/// Prefer this in tight loops or when emitting lots of output, where repeated
/// `println!` calls would otherwise lock and write frequently.
#[inline]
pub fn buf_stdout() -> BufStdout {
  BufWriter::new(io::stdout().lock())
}

/// Creates a buffered, locked `stderr` writer.
///
/// Useful for high-volume diagnostics or logging to stderr with fewer
/// syscalls.
#[inline]
pub fn buf_stderr() -> BufStderr {
  BufWriter::new(io::stderr().lock())
}
