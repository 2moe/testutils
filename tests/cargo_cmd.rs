#[test]
fn cargo_cmd() {
  use testutils::{
    get_pkg_name,
    os_cmd::presets::{
      CargoCmd,
      cargo_build::{BuildStd, BuildStdFeatures, RustcTarget},
    },
  };

  let vec = CargoCmd::default()
    .with_nightly(true)
    .with_pkg(get_pkg_name!().into())
    .with_target(RustcTarget::aarch64_linux_android.into())
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
}
