# TestUtils

## Features

- all
  - 所有功能
- std
  - 啟用後，無法在 no_std 環境中執行。
- ext_traits
  - 包含了 BoolExt (為 bool 型別提供 `.ok_or_else()`)
  - `pub use tap::{Pipe, Tap};`
- tiny_container
  - `pub type TString<const N: usize> = tinyvec_string::TinyString<[u8; N]>;`
  - 提供了 Formattable trait (讓 TString 可以透過 `format()` 來格式化)
  - 提供了 IntoBoxedStr trait (`into_boxed_str()`)
- os_cmd
  - 提供了一些可配置的 Cargo 命令的結構體
    - 比如 CargoDoc, CargoCmd
  - 提供了 RunnableCommand Trait

## macros

### dbg_ref

`dbg_ref!` 會輸出傳入引數的型別以及值，主要用於除錯。

與 `std::dbg!` 不同的是：它透過 `log::debug!` 來輸出，而不是直接 (用`eprintln!`) 輸出到 stderr。

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

### `dbg!`

透過 `eprintln!` 而不是 `log::debug!` 來輸出。

```rust
use testutils::dbg;

let x = 42;
dbg!(x); // Prints: x: i32 = 42

let y = "hello";
dbg!(y); // Prints: y: &str = "hello"
```

### generate_struct_arr

`generate_struct_arr!` 可以將結構體轉換為陣列。

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

    // 假設你在 lib.rs 中定義了 `#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]`，那麼 `cfg` 就是 `__unstable_doc`
      --cfg  __unstable_doc

    // 包含非“真正公開” (如 pub(crate) ) items in the generated documentation.
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

對於 cargo rustdoc 命令，我們不需要從 raw string 中構造 runner，我們可以直接使用現有的 CargoDoc 結構體。

預設的 CargoDoc 是：

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

我們可以用 `CargoDoc::default()` 來初始化一個包含預設配置的 CargoDoc 結構體，然後匯入 `RunnableCommand` Trait, 最後呼叫 `.run()` 來執行命令。

```rust
use std::io;
use testutils::{
  get_pkg_name,
  os_cmd::{RunnableCommand, presets::CargoDoc},
};

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  CargoDoc::default()
    .with_pkg(get_pkg_name!())
    .run()
}
```

我們可以透過 `.with_[欄位名稱]` 來配置它

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

## CargoCmd

### Default

```rust
CargoCmd {
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
    all_packages: false,
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
