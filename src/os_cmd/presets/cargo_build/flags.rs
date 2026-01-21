use core::fmt::Display;

use getset::{Getters, WithSetters};
use tap::Pipe;

use crate::os_cmd::{MiniStr, fmt_compact};

mod relocation_model;
pub use relocation_model::RelocationModel;

mod code_model;
pub use code_model::CodeModel;

mod linker_flavor;
pub use linker_flavor::LinkerFlavor;

/// Converts an identifier to a kebab-case key and its corresponding value.
///
/// ```
/// use testutils::ident_to_kebab_kv;
///
/// let foo_bar = "Foo";
/// let (key, value) = ident_to_kebab_kv!(foo_bar);
/// assert_eq!(key, "foo-bar");
/// assert_eq!(value, "Foo");
/// ```
#[macro_export]
macro_rules! ident_to_kebab_kv {
  ($ident:ident) => {
    (
      stringify!($ident)
        .chars()
        .map(|c| match c {
          '_' => '-',
          c => c,
        })
        .collect::<$crate::os_cmd::MiniStr>(),
      $ident,
    )
  };
}

/// try_into_mini_arg(flag, value) =>`format_args!({flag}={value})`
///
/// if value.is_empty() => None
pub fn try_into_mini_arg<D, S>(flag: D, value: S) -> Option<MiniStr>
where
  D: Display,
  S: AsRef<str>,
{
  match value.as_ref() {
    "" => None,
    v => fmt_compact!("{flag}={v}").into(),
  }
}

#[derive(Debug, Clone, WithSetters, Getters)]
#[getset(set_with = "pub", get = "pub with_prefix")]
/// Represents various flags used for configuring Rust compilation.
///
/// - `crt_static`
///   - `Some(true)` => `["-C", "target-feature=+crt-static"]`
///   - `Some(false)` => `["-C", "target-feature=-crt-static"]`
///   - `None` => `[]`
///
/// - prefer_dynamic
///   - `Some(true)` => `["-C", "prefer-dynamic=true"]`
///   - `Some(false)` => `["-C", "prefer-dynamic=false"]`
///   - `None` => `[]`
///
/// - `linker`: The linker to be used.
///   - "" => `[]`
///   - lld => `["-C", "linker=lld"]`
///
/// - `relocation_model`: static, pic, pie, etc.
/// - `code_model`: "tiny", "small", "kernel", "medium", "large"
///
/// - `codegen_units`:
///   - Some(u) => `["-C", "codegen-units={u}"]`
///   - None => `[]`
///
/// - `native_target_cpu`:
///   - Some(true) => `["-C", "target-cpu=native"]`
///   - Some(false) => `["-C", "target-cpu=generic"]`
///   - None => `[]`
///
/// - `other_flags`: Additional flags for the Rust compiler.
///
/// See also: [The rustc book](https://doc.rust-lang.org/rustc/codegen-options/index.html)
pub struct RustFlags {
  crt_static: Option<bool>,
  prefer_dynamic: Option<bool>,
  linker: MiniStr,
  linker_flavor: LinkerFlavor,
  link_self_contained: Option<bool>,
  relocation_model: RelocationModel,
  code_model: CodeModel,
  codegen_units: Option<usize>,
  native_target_cpu: Option<bool>,
  other_flags: Box<[MiniStr]>,
}

impl RustFlags {
  /// Collects flags into a Vec
  ///
  ///
  /// ## Example
  ///
  /// ```
  /// use tap::Pipe;
  /// use testutils::os_cmd::{
  ///   collect::collect_to_ministr_slice,
  ///   presets::cargo_build::flags::{LinkerFlavor, RustFlags},
  /// };
  ///
  /// let flags = RustFlags::default()
  ///   .with_crt_static(false.into())
  ///   .with_prefer_dynamic(true.into())
  ///   .with_linker_flavor(LinkerFlavor::GNUbinutilsLLVMLLD)
  ///   .with_other_flags(
  ///     [
  ///       "-C",
  ///       "link-arg=-ffunction-sections",
  ///       "-C",
  ///       "link-arg=-fdata-sections",
  ///     ]
  ///     .into_iter()
  ///     .pipe(collect_to_ministr_slice),
  ///   )
  ///   .into_vec();
  ///
  /// assert_eq!(
  ///   flags,
  ///   [
  ///     "-C",
  ///     "target-feature=-crt-static",
  ///     "-C",
  ///     "prefer-dynamic=true",
  ///     "-C",
  ///     "linker-flavor=ld.lld",
  ///     "-C",
  ///     "link-arg=-ffunction-sections",
  ///     "-C",
  ///     "link-arg=-fdata-sections",
  ///   ]
  /// );
  /// ```
  pub fn into_vec(self) -> Vec<MiniStr> {
    let Self {
      crt_static,
      prefer_dynamic,
      linker,
      linker_flavor,
      link_self_contained,
      relocation_model,
      code_model,
      codegen_units,
      native_target_cpu,
      other_flags,
    } = self;

    let gen_bool_flag = |kv: (MiniStr, Option<bool>)| {
      let (ref k, v) = kv;
      v.and_then(|b| try_into_mini_arg(k, fmt_compact!("{b}")))
    };

    let crt_static = crt_static.map(|b| {
      if b { "+" } else { "-" }
        .pipe(|sym| fmt_compact!("target-feature={sym}crt-static"))
    });

    let codegen_units = codegen_units
      .and_then(|u| try_into_mini_arg("codegen-units", fmt_compact!("{u}")));

    let native_target_cpu = native_target_cpu.and_then(|b| {
      try_into_mini_arg("target-cpu", if b { "native" } else { "generic" })
    });

    [
      crt_static,
      gen_bool_flag(ident_to_kebab_kv! {prefer_dynamic}),
      try_into_mini_arg("linker", linker),
      linker_flavor.into(),
      gen_bool_flag(ident_to_kebab_kv! {link_self_contained}),
      relocation_model.into(),
      code_model.into(),
      codegen_units,
      native_target_cpu,
    ]
    .into_iter()
    .flatten()
    .flat_map(|x| ["-C".into(), x])
    .chain(other_flags)
    .collect()
  }
}

impl Default for RustFlags {
  /// Default
  ///
  /// ```ignore
  /// RustFlags {
  ///     crt_static: None,
  ///     prefer_dynamic: None,
  ///     linker: "",
  ///     linker_flavor: Ignore,
  ///     link_self_contained: None,
  ///     relocation_model: Ignore,
  ///     code_model: Ignore,
  ///     codegen_units: None,
  ///     native_target_cpu: None,
  ///     other_flags: [],
  /// }
  /// ```
  fn default() -> Self {
    Self {
      other_flags: Default::default(),
      crt_static: None,
      linker: "".into(),
      prefer_dynamic: None,
      link_self_contained: None,
      linker_flavor: Default::default(),
      code_model: Default::default(),
      relocation_model: Default::default(),
      codegen_units: None,
      native_target_cpu: None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::os_cmd::collect::collect_to_ministr_slice;

  #[ignore]
  #[test]
  fn test_default_flags() {
    let flags = RustFlags::default();
    dbg!(&flags);
    assert_eq!(flags.other_flags.len(), 0);
    assert_eq!(flags.crt_static, None);
    assert_eq!(flags.prefer_dynamic, None);
    assert_eq!(flags.linker, "");
    assert_eq!(flags.link_self_contained, None);
    assert_eq!(flags.codegen_units, None);
    assert_eq!(flags.native_target_cpu, None);
  }
  #[ignore]
  #[test]
  fn test_rust_flags_into_vec() {
    let flags = RustFlags::default()
      .with_code_model(CodeModel::Large)
      .with_crt_static(false.into())
      .with_prefer_dynamic(true.into())
      .with_linker("ldd".into())
      .with_other_flags(
        ["-L", "/lib"]
          .into_iter()
          .pipe(collect_to_ministr_slice),
      )
      .into_vec();

    eprintln!("{}", flags.join(" "))
  }

  #[ignore]
  #[test]
  fn test_into_vec2() {
    let flags = RustFlags::default()
      .with_crt_static(false.into())
      .with_prefer_dynamic(true.into())
      .with_linker_flavor(LinkerFlavor::GNUbinutilsLLVMLLD)
      .with_other_flags(
        [
          "-C",
          "link-arg=-ffunction-sections",
          "-C",
          "link-arg=-fdata-sections",
        ]
        .into_iter()
        .pipe(collect_to_ministr_slice),
      )
      .into_vec();
    assert_eq!(
      flags,
      [
        "-C",
        "target-feature=-crt-static",
        "-C",
        "prefer-dynamic=true",
        "-C",
        "linker-flavor=ld.lld",
        "-C",
        "link-arg=-ffunction-sections",
        "-C",
        "link-arg=-fdata-sections",
      ]
    );
  }
}
