use getset::{CopyGetters, WithSetters};
use tap::Pipe;

use crate::{
  os_cmd::{CommandRepr, presets::TinyCfg},
  tiny_container::IntoBoxedSlice,
};

#[derive(Debug, Clone, WithSetters, CopyGetters)]
#[getset(set_with = "pub", get_copy = "pub with_prefix")]
/// Configurable cargo fmt command.
///
/// ```ignore
/// [
///   "cargo", "+nightly", "fmt",
/// ]
/// ```
///
/// ## Example
///
/// ```
/// use testutils::os_cmd::presets::CargoFmt;
/// use testutils::os_cmd::CommandRepr;
///
/// let cmd = CargoFmt::default()
///   .with_nightly(false) // default is true
/// ;
/// // dbg!(&cmd);
/// assert!(!cmd.get_nightly()); // false
///
/// let _command: CommandRepr = cmd.into();
/// ```
pub struct CargoFmt {
  nightly: bool,
}

impl Default for CargoFmt {
  fn default() -> Self {
    Self { nightly: true }
  }
}

impl From<CargoFmt> for CommandRepr<'_> {
  /// ```ignore
  /// [
  ///   "cargo", "+nightly", "fmt",
  /// ].into_boxed_slice()
  /// ```
  #[allow(clippy::unnecessary_lazy_evaluations)]
  fn from(value: CargoFmt) -> Self {
    use core::iter::once;

    "cargo"
      .pipe(once)
      .chain(
        value
          .get_nightly()
          .then(|| "+nightly"),
      )
      .chain(["fmt"])
      .collect::<TinyCfg<3>>()
      .into_boxed_slice()
      .pipe(CommandRepr::Slice)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[ignore]
  #[test]
  fn test_cargo_fmt_cmd() {
    let cmd: CommandRepr = CargoFmt::default()
      .with_nightly(true)
      .into();
    dbg!(&cmd);
  }
}
