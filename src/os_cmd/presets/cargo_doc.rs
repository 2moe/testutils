use getset::{Getters, Setters, WithSetters};
use tap::Pipe;

use crate::os_cmd::{CommandRepr, RunnableCommand, Runner, presets::StrVec};
impl<'a> RunnableCommand<'a> for CargoDoc<'a> {}

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
/// assert_eq!(cmd.get_custom_cfg(), &"docsrs");
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

/// Converts an identifier into a `(name, value)` pair.
///
/// This macro expands to `(stringify!(ident), ident)`.
///
/// # Examples
///
/// ```
/// use testutils::ident_value_pair;
///
/// let pkg = "cargo";
/// let res = ident_value_pair!(pkg);
/// assert_eq!(res, ("pkg", "cargo"));
///
/// let num = 3;
/// let res2 = ident_value_pair!(num);
/// assert_eq!(res2, ("num", 3));
/// ```
#[macro_export]
macro_rules! ident_value_pair {
  ($value:ident) => {
    (stringify!($value), $value)
  };
}

/// - pkg
///   - "" => `[]`
///   - value => `["--package", value]`
///
/// - custom_cfg
///   - "" => `[]`
///   - value => `["--cfg", value]`
fn concat_tinycfg<'a>(arg_tup: (&str, &'a str)) -> StrVec<'a, 2> {
  let (field_name, value) = arg_tup;

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
  /// `CargoDoc<'_>` => `TinyVec<[&str; 11]>`
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
      .chain(ident_value_pair!(pkg).pipe(concat_tinycfg))
      .chain(all_features.then(|| "--all-features"))
      .chain(open.then(|| "--open"))
      .chain(["--"])
      .chain(ident_value_pair!(custom_cfg).pipe(concat_tinycfg))
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
