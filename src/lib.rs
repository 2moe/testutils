#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(__unstable_doc, feature(doc_auto_cfg, doc_notable_trait))]
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

- **tiny_container**
  Compact string (<=N: Inline(Stack), >N: Overflow to Heap):
  - `TString<const N: usize>` type alias for `TinyString<[u8; N]>`
  - `Formattable` trait
    - Enables `format` support for `TString`
  - `IntoBoxedStr` trait
    - Adds `.into_boxed_str()` conversion

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

#[cfg(feature = "tiny_container")]
/// TString & TinyVec
pub mod tiny_container;

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

/// `env!("CARGO_PKG_NAME")`
#[macro_export]
macro_rules! get_pkg_name {
  () => {
    env!("CARGO_PKG_NAME")
  };
}

/// Similar to the `dbg!` macro, but inspects values by reference without moving
/// them. This allows debugging values without transferring ownership, while
/// showing the underlying value's type information (not reference types).
///
/// This macro uses [`log::debug!`] internally, so you must:
/// 1. Configure a logger (e.g., `env_logger`) with debug level enabled
/// 2. Initialize the logger before use
///
/// ## Key Differences from `dbg!`
/// - Returns `()` instead of the original value
/// - Requires explicit logger initialization
///
/// ## Examples
///
/// Basic usage with automatic type deduction
///
/// ```
/// use testutils::dbg_ref;
/// // Must initialize logger first:
/// // env_logger::builder().filter_level(log::LevelFilter::Debug).init();
///
/// let counter = 42;
/// dbg_ref!(counter); // [DEBUG] counter: i32 = 42
///
/// let message = "Hello";
/// dbg_ref!(message); // [DEBUG] message: &str = "Hello"
///
/// let values = vec![(1, "a"), (2, "b")];
/// dbg_ref!(values); // [DEBUG] values: alloc::vec::Vec<(i32, &str)> = [...]
/// ```
///
/// Debugging multiple values
///
/// ```
/// use testutils::dbg_ref;
/// let width = 30;
/// let label = "size";
///
/// dbg_ref!(width, label);
/// // Outputs:
/// // [DEBUG] width: i32 = 30
/// // [DEBUG] label: &str = "size"
/// ```
///
/// ## Implementation Notes
/// 1. Uses `core::any::type_name_of_val` for type information
/// 2. Formats output as: `{variable_name}: {type} = {debug_representation}`
/// 3. Multiple arguments generate separate log entries
#[macro_export]
macro_rules! dbg_ref {
  ($val:expr $(,)?) => {{
    match &$val {
      tmp => {
        log::debug!(
          "{name}: {type_name} = {tmp:?}",
          name = stringify!($val),
          type_name = core::any::type_name_of_val(tmp),
        );
      }
    }
  }};
  ($($val:expr),+ $(,)?) => {
    ($($crate::dbg_ref!($val)),+,)
  };
}

/// Outputs the information of the expression(s) to stderr.
///
/// ```
/// use testutils::dbg;
/// let width = 30;
/// let label = "size";
///
/// dbg!(width, label);
/// // Outputs:
/// //  width: i32 = 30
/// //  label: &str = "size"
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! dbg {
  ($val:expr $(,)?) => {{
    match &$val {
      tmp => {
        eprintln!(
          "\u{1B}[35m{name}\u{1B}[0m: \u{1B}[33m{type_name}\u{1B}[0m = {tmp:?}",
          name = stringify!($val),
          type_name = core::any::type_name_of_val(tmp),
        );
      }
    }
  }};
  ($($val:expr),+ $(,)?) => {
    ($($crate::dbg_ref!($val)),+,)
  };
}

/// Generates a list of tuples containing field names and their values
///
/// ## Example
///
/// ```
/// use testutils::generate_struct_arr;
///
/// struct BuildStd {
///   std: bool,
///   core: bool,
///   alloc: bool,
/// }
///
/// let b = BuildStd {
///   std: false,
///   core: true,
///   alloc: true,
/// };
///
/// let arr = generate_struct_arr![ b => core, alloc, std ];
/// assert_eq!(
///   arr,
///   [("core", true), ("alloc", true), ("std", false)]
/// );
/// ```
#[macro_export]
macro_rules! generate_struct_arr {
  ($self:ident => $( $field:ident ),* $(,)? ) => {{
    [
      $( ( stringify!($field), $self.$field ), )*
    ]
  }};
}
