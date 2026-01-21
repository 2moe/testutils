use getset::{Getters, Setters, WithSetters};
use tap::Pipe;

use crate::os_cmd::{CommandRepr, RunnableCommand, Runner, presets::StrVec};

#[derive(Debug, Clone, WithSetters, Setters, Getters)]
#[getset(set_with = "pub", set = "pub", get = "pub with_prefix")]
/// Configurable cargo rustdoc command.
///
/// This struct allows you to configure and generate a `cargo rustdoc` command
/// with various options. It supports specifying the package, opening the
/// documentation after generation, and including private items in the
/// documentation.
///
/// ```ignore
/// [
///   "cargo", "+nightly", "rustdoc",
///   "--package", pkg, // Automatically disables `--package` when pkg is an empty string.
///   "--all-features", "--open",
///   "--",
///   "--cfg", "docsrs", // The default custom_cfg is "docsrs". When it is empty, `--cfg` is automatically disabled.
///   "--document-private-items",
/// ]
/// ```
///
/// ## Example
///
/// ```
/// use testutils::{
///   get_pkg_name,
///   os_cmd::{CommandRepr, presets::CargoDoc},
/// };
///
/// let cmd = CargoDoc::default()
///   .with_pkg(get_pkg_name!())
///   .with_nightly(true) // default is true
///   .with_open(false) // default is true
///   .with_enable_private_items(false) // default is true
/// ;
/// // dbg!(&cmd);
/// assert_eq!(cmd.get_custom_cfg(), "docsrs");
/// assert!(cmd.get_nightly()); // true
/// assert!(!cmd.get_open()); // false
/// assert!(!cmd.get_enable_private_items()); // false
///
/// let _command: CommandRepr = cmd.into();
/// ```
pub struct CargoDoc<'a> {
  pkg: &'a str,
  custom_cfg: &'a str,
  nightly: bool,
  all_features: bool,
  open: bool,
  enable_private_items: bool,
  extra_args: Box<[&'a str]>,
}

/// generate_arg!(pkg) => concat_tinycfg("pkg", pkg) => `["--package", pkg]`
macro_rules! generate_arg {
  ($value:ident) => {
    concat_tinycfg(stringify!($value), $value)
  };
}
/// - pkg
///   - "" => `[]`
///   - value => `["--package", value]`
///
/// - custom_cfg
///   - "" => `[]`
///   - value => `["--cfg", value]`
fn concat_tinycfg<'a>(field_name: &str, value: &'a str) -> StrVec<'a, 2> {
  let get_arg = || match field_name {
    "pkg" => "--package",
    _ => "--cfg",
  };

  match value {
    "" => StrVec::new(),
    v => [get_arg(), v].into(),
  }
}

impl<'a> CargoDoc<'a> {
  /// This function processes according to the configuration of the CargoDoc
  /// struct fields, collects the result into a StrVec<11>.
  #[allow(clippy::unnecessary_lazy_evaluations)]
  pub fn into_tinyvec(self) -> StrVec<'a, 11> {
    let CargoDoc {
      pkg,
      custom_cfg,
      nightly,
      all_features,
      open,
      enable_private_items,
      extra_args,
    } = self;

    "cargo"
      .pipe(core::iter::once)
      .chain(nightly.then(|| "+nightly"))
      .chain(["rustdoc"])
      .chain(generate_arg!(pkg))
      .chain(all_features.then(|| "--all-features"))
      .chain(open.then(|| "--open"))
      .chain(["--"])
      .chain(generate_arg!(custom_cfg))
      .chain(enable_private_items.then(|| "--document-private-items"))
      .chain(extra_args)
      .collect::<StrVec<11>>()
  }
}

impl<'a> From<CargoDoc<'a>> for CommandRepr<'a> {
  /// CargoDoc => `CommandRepr::Slice`
  fn from(value: CargoDoc<'a>) -> Self {
    value
      .into_tinyvec()
      .into_boxed_slice()
      .into()
    // .pipe(CommandRepr::Slice)
  }
}

impl<'a> From<CargoDoc<'a>> for Runner<'a> {
  fn from(value: CargoDoc<'a>) -> Self {
    Self::default() //
      .with_command(value.into())
  }
}

impl Default for CargoDoc<'_> {
  /// Default:
  ///
  /// ```ignore
  /// CargoDoc {
  ///     pkg: "",
  ///     custom_cfg: "docsrs",
  ///     nightly: true,
  ///     all_features: true,
  ///     open: true,
  ///     enable_private_items: true,
  ///     extra_args: Default::default(),
  /// }
  /// ```
  fn default() -> Self {
    Self {
      nightly: true,
      pkg: "",
      custom_cfg: "docsrs",
      all_features: true,
      open: true,
      enable_private_items: true,
      extra_args: Default::default(),
    }
  }
}

impl<'a> RunnableCommand<'a> for CargoDoc<'a> {}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{get_pkg_name, os_cmd::Runner};

  #[test]
  #[ignore]
  fn test_cargo_doc_cmd() {
    let cmd = CargoDoc::default().with_pkg(get_pkg_name!());
    // dbg!(&cmd);
    assert_eq!(cmd.pkg, "testutils");
    assert_eq!(cmd.custom_cfg, "docsrs");
    assert!(cmd.nightly);
    assert!(cmd.open);
    assert!(cmd.enable_private_items);

    let cmd_repr: CommandRepr = cmd.into();
    let _runner: Runner = cmd_repr.into();
  }

  #[ignore]
  #[test]
  fn show_cargo_doc_default() {
    CargoDoc::default().pipe(|x| dbg!(x));
  }
}
