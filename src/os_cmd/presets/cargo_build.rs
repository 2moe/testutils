use core::fmt::Display;
use std::env;

use compact_str::ToCompactString;
use getset::{Getters, WithSetters};
use tap::{Pipe, Tap};

use crate::os_cmd::{CommandRepr, MiniStr, RunnableCommand, Runner, fmt_compact};

mod sub_cmd;
pub use sub_cmd::SubCmd;

mod cargo_profile;
pub use cargo_profile::CargoProfile;

pub mod flags;

mod target_list;
pub use target_list::RustcTarget;

mod build_std;
pub use build_std::BuildStd;

mod build_std_features;
pub use build_std_features::BuildStdFeatures;

#[derive(Debug, Clone, WithSetters, Getters)]
#[getset(set_with = "pub", get = "pub with_prefix")]
/// Configurable cargo build command.
///
/// ```
/// use testutils::{
///   get_pkg_name,
///   os_cmd::{
///     Runner,
///     presets::{
///       CargoCmd,
///       cargo_build::{BuildStd, BuildStdFeatures},
///     },
///   },
/// };
///
/// let vec = CargoCmd::default()
///   .with_nightly(true)
///   .with_pkg(get_pkg_name!().into())
///   .with_build_std(
///     BuildStd::default()
///       .with_alloc(true)
///       .with_core(true),
///   )
///   .with_build_std_features(
///     BuildStdFeatures::default().with_panic_immediate_abort(true),
///   )
///   .into_vec();
/// assert_eq!(
///   vec,
///   [
///     "cargo",
///     "+nightly",
///     "build",
///     "--profile=release",
///     "--package=testutils",
///     "-Z",
///     "build-std=core,alloc",
///     "-Z",
///     "build-std-features=panic_immediate_abort"
///   ]
/// );
///
/// let _runner: Runner = vec.into();
/// // runner.run();
/// ```
///
/// See also: [The cargo book](https://doc.rust-lang.org/cargo/reference/unstable.html)
pub struct CargoCmd {
  rust_flags: flags::RustFlags,
  // use_os_cmd_to_set_env: bool,
  nightly: bool,
  cargo: MiniStr,
  sub_command: SubCmd,
  profile: CargoProfile,
  pkg: MiniStr,
  target: RustcTarget,
  all_packages: bool,
  all_features: bool,
  no_default_features: bool,
  features: Box<[MiniStr]>,
  build_std: BuildStd,
  build_std_features: BuildStdFeatures,
  other_args: Box<[MiniStr]>,
}

impl RunnableCommand<'_> for CargoCmd {}

impl Default for CargoCmd {
  /// Default:
  ///
  /// ```ignore
  /// CargoCmd {
  ///     rust_flags: RustFlags {
  ///         crt_static: None,
  ///         prefer_dynamic: None,
  ///         linker: "",
  ///         linker_flavor: Ignore,
  ///         link_self_contained: None,
  ///         relocation_model: Ignore,
  ///         code_model: Ignore,
  ///         codegen_units: None,
  ///         native_target_cpu: None,
  ///         other_flags: [],
  ///     },
  ///     nightly: false,
  ///     cargo: "cargo",
  ///     sub_command: Build,
  ///     profile: Release,
  ///     pkg: "",
  ///     target: default,
  ///     all_packages: false,
  ///     all_features: false,
  ///     no_default_features: false,
  ///     features: [],
  ///     build_std: BuildStd {
  ///         build_default: false,
  ///         std: false,
  ///         core: false,
  ///         alloc: false,
  ///         panic_abort: false,
  ///         panic_unwind: false,
  ///         test: false,
  ///         proc_macro: false,
  ///     },
  ///     build_std_features: BuildStdFeatures {
  ///         panic_immediate_abort: false,
  ///         panic_unwind: false,
  ///         backtrace: false,
  ///         optimize_for_size: false,
  ///         llvm_libunwind: false,
  ///         system_llvm_libunwind: false,
  ///         debug_refcell: false,
  ///         debug_typeid: false,
  ///         std_detect_file_io: false,
  ///         std_detect_dlsym_getauxval: false,
  ///         std_detect_env_override: false,
  ///         windows_raw_dylib: false,
  ///     },
  ///     other_args: [],
  /// }
  /// ```
  fn default() -> Self {
    Self {
      rust_flags: Default::default(),
      nightly: false,
      cargo: "cargo".into(),
      sub_command: Default::default(),
      profile: Default::default(),
      pkg: "".into(),
      target: Default::default(),
      all_packages: false,
      all_features: false,
      no_default_features: false,
      features: Default::default(),
      build_std: Default::default(),
      build_std_features: Default::default(),
      other_args: Default::default(),
    }
  }
}

/// try_into_long_arg(flag, value) =>`"--{flag}={value}"`
///
/// if value.is_empty() => None
pub fn try_into_long_arg<D, S>(flag: D, value: S) -> Option<MiniStr>
where
  D: Display,
  S: AsRef<str>,
{
  match value.as_ref() {
    "" => None,
    v => fmt_compact!("--{flag}={v}").into(),
  }
}

pub trait ArgConverter {
  type ArgsIter: Iterator<Item = MiniStr>;
  // = core::array::IntoIter<compact_str::CompactString>;

  fn to_args(&self) -> Self::ArgsIter;
}

impl CargoCmd {
  /// Collects all CargoCmd options into a vec
  #[allow(clippy::unnecessary_lazy_evaluations)]
  pub fn into_vec(self) -> Vec<MiniStr> {
    let CargoCmd {
      rust_flags,
      cargo,
      sub_command,
      nightly,
      profile,
      pkg,
      target,
      all_packages,
      all_features,
      no_default_features,
      features,
      build_std,
      build_std_features,
      other_args,
    } = self;

    let rust_flags_value = rust_flags
      .into_vec()
      .join(" ")
      .tap(|x| log::debug!("setenv: RUSTFLAGS={x}"));

    unsafe { env::set_var("RUSTFLAGS", rust_flags_value) }

    match cargo {
      c if c.is_empty() => "cargo".into(),
      c => c,
    }
    .pipe(core::iter::once)
    .chain(nightly.then(|| "+nightly".into()))
    .chain(
      sub_command
        .as_str()
        .to_compact_string()
        .pipe(Some),
    )
    // --profile {profile}
    .chain(try_into_long_arg("profile", profile))
    // --package {pkg}
    .chain(try_into_long_arg("package", pkg))
    // --workspace
    .chain(all_packages.then(|| "--workspace".into()))
    // --target {target.as_ref()}
    .chain(try_into_long_arg("target", target))
    .chain(all_features.then(|| "--all-features".into()))
    .chain(no_default_features.then(|| "--no-default-features".into()))
    // --features {features.join(",")}
    .chain(match features {
      x if x.is_empty() => None,
      feats => Some(fmt_compact!("--features={}", feats.join(","))),
    })
    // --build-std {build_std.to_args()}
    .chain(build_std.to_args())
    .chain(build_std_features.to_args())
    .chain(other_args)
    .collect()
  }
}

impl From<CargoCmd> for CommandRepr<'_> {
  fn from(value: CargoCmd) -> Self {
    value
      .into_vec()
      .into_boxed_slice()
      .pipe(CommandRepr::OwnedSlice)
  }
}

impl From<CargoCmd> for Runner<'_> {
  fn from(value: CargoCmd) -> Self {
    Self::default() //
      .with_command(value.into())
  }
}

#[cfg(test)]
mod tests {
  use tap::Pipe;

  // use super::*;
  use crate::get_pkg_name;

  #[test]
  #[ignore]
  fn test_cargo_build_command() {
    use crate::{
      get_pkg_name,
      os_cmd::{
        Runner,
        presets::{
          CargoCmd,
          cargo_build::{BuildStd, BuildStdFeatures, RustcTarget},
        },
      },
    };

    let vec = CargoCmd::default()
      .with_nightly(true)
      .with_pkg(get_pkg_name!().into())
      .with_target(RustcTarget::aarch64_linux_android)
      .with_build_std(
        BuildStd::default()
          .with_alloc(true)
          .with_core(true),
      )
      .with_build_std_features(
        BuildStdFeatures::default().with_panic_immediate_abort(true),
      )
      .into_vec();
    assert_eq!(
      vec,
      [
        "cargo",
        "+nightly",
        "build",
        "--profile=release",
        "--package=testutils",
        "--target=aarch64-linux-android",
        "-Z",
        "build-std=core,alloc",
        "-Z",
        "build-std-features=panic_immediate_abort"
      ]
    );
    // dbg_ref!(vec);
    println!("{vec:?}");
    let _runner: Runner = vec.into();
    // runner.run();
  }

  #[ignore]
  #[test]
  fn show_default_cargo_build() {
    use crate::os_cmd::presets::CargoCmd;
    CargoCmd::default().pipe(|x| dbg!(x));
  }
}
