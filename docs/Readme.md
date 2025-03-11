# TestUtils

A utility library providing various helper functions, macros, and tools for Rust development.

[![testutils.crate](https://img.shields.io/crates/v/testutils?label=testutils)](https://crates.io/crates/testutils)

[![Documentation](https://docs.rs/testutils/badge.svg)](https://docs.rs/testutils)

[![Apache-2 licensed](https://img.shields.io/crates/l/testutils.svg)](../License)

[中文](./Readme-zh.md)

## Features

- **all**
  All available features enabled

- **std**
  Enables standard library support. When enabled, the crate cannot be used in `no_std` environments.

- **ext_traits**
  Additional trait extensions:
  - `BoolExt` - Adds `.ok_or_else()` method for `bool` type
  - Re-exports `Pipe` and `Tap` traits from `tap` crate

- **tiny_container**
  Compact string (<=N: Inline(Stack), >N: Overflow to Heap):
  - `TString<const N: usize>` type alias for `TinyString<[u8; N]>`
  - `Formattable` trait
    - Enables `format` support for `TString`
  - `IntoBoxedStr` trait
    - Adds `.into_boxed_str()` conversion

- **os_cmd**
  - Configurable command builders:
    - Preconfigured cargo command structs (e.g., `CargoDoc`, `CargoCmd`)
    - Cross-platform command execution utilities
  - `RunnableCommand` trait
    - Provides `.run()`

## Macros

### `dbg_ref!`

Prints debug information using `log::debug!` instead of direct stderr output. Displays both the type and value of the passed argument.

**Comparison with `std::dbg!`**:

- Uses structured logging instead of direct stderr output
- Requires logger initialization (e.g., `env_logger`)
- Shows full type paths for better diagnostics

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

Similar to `dbg_ref!`, but outputs content using `eprintln!` instead of `log::debug!`.

```rust
use testutils::dbg;

let x = 42;
dbg!(x); // Prints: x: i32 = 42

let y = "hello";
dbg!(y); // Prints: y: &str = "hello"
```

### `generate_struct_arr!`

Converts a struct into an array of field-value tuples. Particularly useful for testing and configuration scenarios.

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

## OS Command Configuration

### Raw String Configuration

Build commands using commented configuration strings with automatic comment stripping:

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

    // Requires cfg matching attributes
    // Assume you define `#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]` in `lib.rs`, then `cfg` is `__unstable_doc`.
      --cfg  __unstable_doc

  // Include non-real-public (e.g., pub(crate) ) items in the generated documentation.
      --document-private-items
    "#,
    pkg = get_pkg_name!()
  )
  .pipe_deref(Runner::from)
  .with_remove_comments(true)
  .run()
}
```

### Preset Command: `CargoDoc`

Preconfigured command builder with sensible defaults:

**Default Configuration**:

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

**Usage Example**:

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

**Configuration Methods**:

```rust
use testutils::{dbg_ref, os_cmd::presets::CargoDoc, traits::Tap};

#[ignore]
#[test]
fn configure_cargo_doc() {
  let cmd = CargoDoc::default()
    .with_pkg("")
    .with_custom_cfg("")
    .with_nightly(true)
    .with_open(false)
    .with_enable_private_items(false)
    .with_all_features(true);

  assert_eq!(
    cmd.into_slice(),
    &["cargo", "+nightly", "rustdoc", "--all-features", "--"]
  );
}
```

**Key Methods**:

- `with_pkg()`: Set package name
- `with_custom_cfg()`: Specify custom cfg attributes
- `with_nightly()`: Toggle nightly toolchain
- `with_open()`: Control browser auto-opening
- `with_enable_private_items()`: Toggle private item inclusion
- `with_all_features()`: Control feature activation

### Preset Command: `CargoCmd`

Configuration struct for `cargo build` command generation.

Used to generate precise cargo command-line arguments.

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

```rust
let vec = CargoCmd::default()
  .with_nightly(true)  // Enables nightly channel features
  .with_pkg(get_pkg_name!().into())  // Macro-based package detection
  .with_target(RustcTarget::aarch64_linux_android)  // enum Target
  .with_build_std(  // Custom stdlib components
    BuildStd::default()
      .with_alloc(true)
      .with_core(true),
  )
  .with_build_std_features(
    BuildStdFeatures::default()
      .with_panic_immediate_abort(true),  // Optimized panic handling
  )
  .into_vec();  // Finalize configuration to argument list

// Verify generated command structure matches expectations
assert_eq!(
  vec,
  [
    "cargo",
    "+nightly",         // Toolchain
    "build",            // Subcommand
    "--profile=release",
    "--package=testutils", // crate name
    "--target=aarch64-linux-android",
    "-Z", "build-std=core,alloc", // Custom std components
    "-Z", "build-std-features=panic_immediate_abort" // Std feature flags
  ]
);

// Convert to command executor
let runner: Runner = vec.into();
// Uncomment to execute:
// runner.run();
```
