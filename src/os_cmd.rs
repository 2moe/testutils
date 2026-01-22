mod collect;
pub use collect::collect_boxed_ministr_slice;

/// Provides configurable command runners such as `CargoDoc` and `CargoCmd`.
pub mod presets;

mod repr;
use alloc::borrow::Cow;
use std::ffi::{OsStr, OsString};

pub use compact_str::{CompactString as MiniStr, format_compact as fmt_compact};
pub use repr::{CommandRepr, collect_raw, remove_comments_and_collect};

mod runner;
pub use runner::{RunnableCommand, Runner, RunnerInspection};

mod process;
pub use process::{CommandSpawner, StdioMode, run_os_cmd as run};

pub mod argv;

mod decoded;
pub use decoded::DecodedText;

pub fn cow_str_into_cow_osstr(s: Cow<'_, str>) -> Cow<'_, OsStr> {
  use Cow::{Borrowed, Owned};

  match s {
    Borrowed(b) => OsStr::new(b).into(),
    Owned(o) => OsString::from(o).into(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[ignore]
  #[test]
  fn test_cmd_runner() {
    let runner =
      Runner::default().with_command(repr::CommandRepr::Raw("cargo +nightly fmt"));
    // assert_eq!(runner.get_command(), CommandRepr::Raw("cargo +nightly fmt"));
    assert_eq!(*runner.get_remove_comments(), true);
    // runner.set_raw("cargo fmt");
    // runner.set_trim(false);
    // runner.set_remove_comments(true);
    // assert_eq!(run_os_cmd(runner.as_str()), Ok(ExitStatus::Success));
  }

  #[ignore]
  #[test]
  fn test_cargo_doc_runner() {
    // CommandRepr::new_cargo_rustdoc_cmd("");
  }
}
