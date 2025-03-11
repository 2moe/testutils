pub mod collect;
/// Provides configurable command runners such as `CargoDoc` and `CargoCmd`.
pub mod presets;

mod repr;
pub use compact_str::{CompactString as MiniStr, format_compact as fmt_compact};
pub use repr::CommandRepr;

mod runner;
pub use runner::{RunnableCommand, Runner};

use crate::tiny_container::TString;

/// on 64bit sys: const N = 28, size = 32 (0x20)
pub type SmallString = TString<28>;

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
