# TestUtils

## Features

- all
  - 所有功能
- std
  - 启用后，无法在 no_std 环境中运行。
- ext_traits
  - 包含了 BoolExt (为 bool 类型提供 `.ok_or_else()`)
  - `pub use tap::{Pipe, Tap};`
- tiny_container
  - `pub type TString<const N: usize> = tinyvec_string::TinyString<[u8; N]>;`
  - 提供了 Formattable trait (让 TString 可以通过 `format()` 来格式化)
  - 提供了 IntoBoxedStr trait (`into_boxed_str()`)
- os_cmd
  - 提供了一些可配置的 Cargo 命令的结构体
    - 比如 CargoDoc, CargoBuild

## macros

### dbg_ref

`dbg_ref!()` 会输出传入参数的类型以及值，主要用于调试。

与 `dbg!()` 不同的是，它内部使用了 `log::debug!()` 来输出，而不是直接 (用`eprintln!`) 输出到 stderr。

```rust
use testutils::dbg_ref;
// env_logger::builder().filter_level(log::LevelFilter::Debug).init();

let x = 42;
dbg_ref!(x); // Prints: [DEBUG] x: i32 = 42

let y = "hello";
dbg_ref!(y); // Prints: [DEBUG] y: &str = "hello"

let z = vec![1, 2, 3];
dbg_ref!(z); // Prints: [DEBUG] z: alloc::vec::Vec<i32> = [1, 2, 3]
```

### generate_struct_arr

`generate_struct_arr!` 可以将结构体转换为数组。

```rust
use testutils::generate_struct_arr;

struct BuildStd {
  std: bool,
  core: bool,
  alloc: bool,
}

let b = BuildStd {
  std: false,
  core: true,
  alloc: true,
};

let arr = generate_struct_arr![ b => core, alloc, std ];
assert_eq!(
  arr,
  [("core", true), ("alloc", true), ("std", false)]
);
```

## os_cmd

### raw string

```rust
use std::io;
use testutils::{get_pkg_name, os_cmd::Runner, traits::Pipe};

#[ignore]
#[test]
fn build_rsdoc() -> io::Result<()> {
  let _ = format!(
    r#"
    // nightly toolchain
      cargo +nightly

    // use rustdoc instead of doc
      rustdoc

    // --package
      -p  {pkg}

      --all-features

      // open browser
      --open

      --

    // 假设你在 lib.rs 中定义了 `#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]`，那么 `cfg` 就是 `__unstable_doc`
      --cfg  __unstable_doc

    // 包含非“真正公开” (如 pub(crate) ) items in the generated documentation.
      --document-private-items
    "#,
    pkg = get_pkg_name!()
  )
  .pipe_deref(Runner::from)
  .with_remove_comments(true) // Because the raw string contains `//` comments, this option (`with_remove_comments`) must be enabled.
  .run()
}
```

### CargoDoc

对比 cargo rustdoc 命令，我们不需要从 raw string 中构造 runner，我们可以直接使用现有的 CargoDoc 结构体。

默认的 CargoDoc 是：

```rust
CargoDoc {
    pkg: "",
    custom_cfg: "__unstable_doc",
    nightly: true,
    all_features: true,
    open: true,
    enable_private_items: true,
}
```

我们可以用 `CargoDoc::default()` 来初始化一个包含默认配置的 CargoDoc 结构体，然后转为 `Runner`, 最后调用 `.run()` 来运行命令。

```rust
use std::io;
use testutils::{
  get_pkg_name,
  os_cmd::{Runner, presets::CargoDoc},
  traits::Pipe,
};

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  CargoDoc::default()
    .with_pkg(get_pkg_name!())
    .pipe(Runner::from)
    .run()
}
```

我们可以通过 `.with_[字段名称]` 来配置它

```rust
use testutils::{dbg_ref, os_cmd::presets::CargoDoc, traits::Tap};

use crate::normal::init_logger;

#[ignore]
#[test]
fn configure_cargo_doc() {
  init_logger();
  CargoDoc::default()
    .with_pkg("")
    .with_custom_cfg("")
    .with_nightly(true)
    .with_open(false)
    .with_enable_private_items(false)
    .with_all_features(true)
    .tap(|x| dbg_ref!(x))
    .into_slice()
    .tap(|x| dbg_ref!(x));

  assert_eq!(
    x.as_slice(),
    &["cargo", "+nightly", "rustdoc", "--all-features", "--"]
  );
}
```

## CargoBuild

### Default

```rust
CargoBuild {
    rust_flags: RustFlags {
        crt_static: None,
        prefer_dynamic: None,
        linker: "",
        linker_flavor: Ignore,
        link_self_contained: None,
        relocation_model: Ignore,
        code_model: Ignore,
        codegen_units: None,
        native_target_cpu: None,
        other_flags: [],
    },
    nightly: false,
    sub_command: Build,
    profile: "release",
    pkg: "",
    target: default,
    all_features: false,
    no_default_features: false,
    features: [],
    build_std: BuildStd {
        build_default: false,
        std: false,
        core: false,
        alloc: false,
        panic_abort: false,
        panic_unwind: false,
        test: false,
        proc_macro: false,
    },
    build_std_features: BuildStdFeatures {
        panic_immediate_abort: false,
        panic_unwind: false,
        backtrace: false,
        optimize_for_size: false,
        llvm_libunwind: false,
        system_llvm_libunwind: false,
        debug_refcell: false,
        debug_typeid: false,
        std_detect_file_io: false,
        std_detect_dlsym_getauxval: false,
        std_detect_env_override: false,
        windows_raw_dylib: false,
    },
    other_args: [],
}
```

### Example

```rust
use testutils::{
  get_pkg_name,
  os_cmd::{
    Runner,
    presets::{
      CargoBuild,
      cargo_build::{BuildStd, BuildStdFeatures, RustcTarget},
    },
  },
};

let vec = CargoBuild::default()
  .with_nightly(true)
  .with_pkg(get_pkg_name!().into())
  .with_target(RustcTarget::aarch64_linux_android)
  .with_build_std(
    BuildStd::default()
      .with_alloc(true)
      .with_core(true),
  )
  .with_build_std_features(
    BuildStdFeatures::default()
      .with_panic_immediate_abort(true),
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
let runner: Runner = vec.into();
// runner.run();
```
