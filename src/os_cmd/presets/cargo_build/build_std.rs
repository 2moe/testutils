use getset::{CopyGetters, WithSetters};

use crate::{
  generate_struct_arr,
  os_cmd::{
    MiniStr, fmt_compact,
    presets::{TinyCfg, cargo_build::ArgConverter},
  },
};

#[derive(Debug, Clone, WithSetters, CopyGetters)]
#[getset(set_with = "pub", get_copy = "pub with_prefix")]
/// Represents the build options for the standard library components.
///
/// By default (`BuildStd::default()`), all options are set to `false`.
///
/// Note: `BuildStd.build_default` is **NOT** equivalent to
/// `BuildStd::default()`
///
/// - **Special Option**: `build_default`: When set to `true` => `["-Z",
///   "build-std"]`, which compiles `core`, `std`, `alloc`, and `proc_macro`.
///   - Note: Other options will override `build_default`
///     - When `build_default` is true and `core` is also true, => `["-Z",
///       "build-std=core"]`
///     - Only when `build_default` is true and all other options are false, =>
///       `["-Z", "build-std"]`
///
/// - `core`: `true` => `["-Z", "build-std=core"]`
/// - `alloc`: `true` => `["-Z", "build-std=alloc"]`
///
/// - When both `core` and `alloc` are `true` => `["-Z",
///   "build-std=core,alloc"]`
///
/// You can control the components to be compiled using the `with_` methods.
///
/// ## Example
///
/// ```
/// use testutils::os_cmd::presets::cargo_build::BuildStd;
/// use testutils::os_cmd::presets::cargo_build::ArgConverter;
///
///   let build_std_with_core_and_alloc = BuildStd::default()
///     .with_std(false)
///     .with_core(true)
///     .with_alloc(true)
///     .with_build_default(true);
///
///   assert_eq!(
///     build_std_with_core_and_alloc
///       .to_args()
///       .next(),
///     Some("-Z".into())
///   );
///
///   assert_eq!(
///     build_std_with_core_and_alloc
///       .to_args()
///       .last(),
///     Some("build-std=core,alloc".into())
///   );
///
///   let build_std_with_default_only = BuildStd::default().with_build_default(true);
///   assert_eq!(
///     build_std_with_default_only
///       .to_args()
///       .last(),
///     Some("build-std".into())
///   );
///
///   let build_std_none_enabled = BuildStd::default();
///   assert_eq!(
///     build_std_none_enabled
///       .to_args()
///       .next(),
///     None
///   );
/// ```
pub struct BuildStd {
  build_default: bool,
  std: bool,
  core: bool,
  alloc: bool,
  panic_abort: bool,
  panic_unwind: bool,
  test: bool,
  proc_macro: bool,
}

impl Default for BuildStd {
  /// Default:
  ///
  /// ```ignore
  /// BuildStd {
  ///   build_default: false,
  ///   std: false,
  ///   core: false,
  ///   alloc: false,
  ///   panic_abort: false,
  ///   panic_unwind: false,
  ///   test: false,
  ///   proc_macro: false,
  /// }
  /// ```
  fn default() -> Self {
    Self {
      build_default: false,
      std: false,
      core: false,
      alloc: false,
      panic_abort: false,
      panic_unwind: false,
      test: false,
      proc_macro: false,
    }
  }
}

impl ArgConverter for BuildStd {
  type ArgsIter = core::iter::Flatten<core::option::IntoIter<[MiniStr; 2]>>;

  /// Converts the `BuildStd` struct to an array of arguments.
  ///
  /// This method generates a list of arguments based on the enabled fields
  /// in the `BuildStd` struct. If no fields are enabled, it returns `None`.
  /// If `build_default` is enabled, it returns the default build arguments.
  fn to_args(&self) -> Self::ArgsIter {
    // **Note:** The `build_default` is special. Do not include `build_default` in
    // the `generate_struct_arr!` macro.
    let components = generate_struct_arr! [ self =>
      std,
      core,
      alloc,
      panic_abort,
      panic_unwind,
      test,
      proc_macro
    ];
    match components
      .into_iter()
      .filter_map(|(name, enabled)| enabled.then_some(name))
      .collect::<TinyCfg<8>>()
    {
      // If there are enabled fields, format them into a build string
      v if !v.is_empty() => [
        "-Z".into(),
        fmt_compact!("build-std={}", v.join(",")), //
      ]
    .into()
    ,
      // If no fields are enabled, check if build_default is enabled
      _ => self
        .build_default
        .then(|| ["-Z", "build-std"].map(Into::into))
    }
    .into_iter()
    .flatten()
  }
}

#[cfg(test)]
mod tests {
  use tap::Pipe;

  use super::*;

  #[ignore]
  #[test]
  fn show_default_build_std() {
    BuildStd::default().pipe(|x| dbg!(x));
  }

  #[ignore]
  #[test]
  fn test_build_std_to_args() {
    let build_std_with_core_and_alloc = BuildStd::default()
      .with_std(false)
      .with_core(true)
      .with_alloc(true)
      .with_build_default(true);

    assert_eq!(
      build_std_with_core_and_alloc
        .to_args()
        .next(),
      Some("-Z".into())
    );

    assert_eq!(
      build_std_with_core_and_alloc
        .to_args()
        .last(),
      Some("build-std=core,alloc".into())
    );

    let build_std_with_default_only = BuildStd::default().with_build_default(true);
    assert_eq!(
      build_std_with_default_only
        .to_args()
        .last(),
      Some("build-std".into())
    );

    let build_std_none_enabled = BuildStd::default();
    assert_eq!(
      build_std_none_enabled
        .to_args()
        .next(),
      None
    );
  }
}
