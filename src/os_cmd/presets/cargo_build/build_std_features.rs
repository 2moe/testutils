use compact_str::format_compact;
use getset::{CopyGetters, WithSetters};

use crate::{
  generate_struct_arr,
  os_cmd::{
    MiniStr,
    presets::{StrVec, cargo_build::ArgConverter},
  },
};

/// Configuration for standard library build features when using `-Zbuild-std`
///
/// By default (via [`Default`] implementation), all features are disabled.
/// Each boolean field represents a specific build feature flag.
///
/// ## Example
///
/// ```
/// use testutils::os_cmd::presets::cargo_build::BuildStdFeatures;
/// use testutils::os_cmd::presets::cargo_build::ArgConverter;
///
/// let feats = BuildStdFeatures::default()
///   .with_panic_unwind(true)
///   .with_optimize_for_size(true);
///
/// assert!(feats.get_panic_unwind());
/// assert!(feats.get_optimize_for_size());
/// assert!(!feats.get_panic_immediate_abort());
///
/// let args = feats.to_args();
///
/// args
///   .last()
///   .map(|x| assert_eq!(x, "build-std-features=panic_unwind,optimize_for_size"));
/// ```
#[derive(Debug, Clone, WithSetters, CopyGetters, Default)]
#[getset(set_with = "pub", get_copy = "pub with_prefix")]
pub struct BuildStdFeatures {
  /// Immediately abort on panic rather than unwinding
  panic_immediate_abort: bool,

  /// Enable panic unwinding support
  panic_unwind: bool,

  /// Enable backtrace support
  backtrace: bool,

  /// Optimize for size rather than speed
  optimize_for_size: bool,

  /// Use LLVM's libunwind for stack unwinding
  llvm_libunwind: bool,

  /// Use system's LLVM libunwind (requires external linkage)
  system_llvm_libunwind: bool,

  /// Enable debug checks for RefCell borrow rules
  debug_refcell: bool,

  /// Enable type ID verification in Any trait
  debug_typeid: bool,

  /// Enable file I/O detection in std::detect
  std_detect_file_io: bool,

  /// Enable dlsym/getauxval detection in std::detect
  std_detect_dlsym_getauxval: bool,

  /// Allow environment variable override for std::detect
  std_detect_env_override: bool,

  /// Use raw dylib linking on Windows platforms
  windows_raw_dylib: bool,
}

impl ArgConverter for BuildStdFeatures {
  type ArgsIter = core::iter::Flatten<core::option::IntoIter<[MiniStr; 2]>>;

  /// Converts enabled features to cargo build arguments
  ///
  /// Returns `Some(["-Z", "build-std-features=..."])` if any features are
  /// enabled, returns `None` if no features are selected.
  ///
  /// The generated arguments are suitable for passing to cargo's unstable `-Z`
  /// flag.
  fn to_args(&self) -> Self::ArgsIter {
    let components = generate_struct_arr! [ self =>
        panic_immediate_abort,
        panic_unwind,
        backtrace,
        llvm_libunwind,
        system_llvm_libunwind,
        optimize_for_size,
        debug_refcell,
        debug_typeid,
        std_detect_file_io,
        std_detect_dlsym_getauxval,
        std_detect_env_override,
        windows_raw_dylib,
    ];

    match components
      .into_iter()
      .filter_map(|(name, enabled)| enabled.then_some(name))
      .collect::<StrVec<12>>()
    {
      // Format enabled features into build-std-features parameter
      v if !v.is_empty() => [
        "-Z".into(),
        format_compact!("build-std-features={}", v.join(",")),
      ]
      .into(),

      // No features enabled - don't generate arguments
      _ => None,
    }
    .into_iter()
    .flatten()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  // #[ignore]
  fn test_default_std_feats() {
    let feats = BuildStdFeatures::default();
    // dbg!(feats);
    assert!(!feats.get_llvm_libunwind());
    assert!(!feats.get_panic_immediate_abort());
  }

  #[ignore]
  #[test]
  fn test_std_feats() {
    let feats = BuildStdFeatures::default()
      .with_panic_unwind(true)
      .with_optimize_for_size(true);

    assert!(feats.get_panic_unwind());
    assert!(feats.get_optimize_for_size());
    assert!(!feats.get_panic_immediate_abort());
    let args = feats.to_args();

    args
      .last()
      .map(|x| assert_eq!(x, "build-std-features=panic_unwind,optimize_for_size"));
  }
}
