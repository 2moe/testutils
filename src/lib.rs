#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
/*!
## Features

- **std**
  Enables standard library support. When enabled, the crate cannot be used in `no_std` environments.

- **bool_ext**
  - Adds `.then_ok_or_else(||{err})` & `.then_ok_or(err)` method for `bool` type

- **print_ext**
  - Provides some printing helpers.

- **re_exports_tap**
  - `pub use tap`

- **os_cmd**
  Configurable command builders:
  - Preconfigured cargo command structs (e.g., `CargoDoc`, `CargoCmd`)
  - Cross-platform command execution utilities
*/
extern crate alloc;

#[cfg(feature = "os_cmd")]
pub mod os_cmd;

mod macros;

#[cfg(feature = "bool_ext")]
pub mod bool_ext;

#[cfg(feature = "print_ext")]
pub mod print_ext;

#[cfg(feature = "re_exports_tap")]
pub use tap;

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
  let elapsed = start.elapsed();

  eprintln!("Time taken: {elapsed:?}")
}
