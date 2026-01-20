#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
/*!
## Features

- **all**
  All available features enabled

- **std**
  Enables standard library support. When enabled, the crate cannot be used in `no_std` environments.

- **ext_traits**
  Additional trait extensions:
  - `BoolExt` - Adds `.ok_or_else()` method for `bool` type
  - Re-exports `Pipe` and `Tap` traits from `tap` crate

- **os_cmd**
  Configurable command builders:
  - Preconfigured cargo command structs (e.g., `CargoDoc`, `CargoCmd`)
  - Cross-platform command execution utilities
*/
extern crate alloc;

#[cfg(feature = "os_cmd")]
pub mod os_cmd;

#[cfg(feature = "ext_traits")]
/// Provides BoolExt(`.ok_or_else()`)
pub mod traits;

mod macros;

/// Runs the given function and prints the elapsed time.
/// It supports stable Rust.
///
/// ## Example
///
/// ```ignore
/// fn bench_foo() {
///   testutils::simple_benchmark(|| {
///     foo() // Your code here...
///   })
/// }
/// ```
#[cfg(feature = "std")]
pub fn simple_benchmark<U, F: FnOnce() -> U>(f: F) {
  let start = std::time::Instant::now();
  f();

  eprintln!("Time taken: {:?}", start.elapsed());
}
