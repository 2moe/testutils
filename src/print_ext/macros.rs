/// Similar to the `std::dbg!` macro, but inspects values by reference without
/// moving them. This allows debugging values without transferring ownership,
/// while showing the underlying value's type information (not reference types).
///
/// This macro uses [`log::debug!`] internally, so you must:
/// 1. Configure a logger (e.g., `env_logger`) with debug level enabled
/// 2. Initialize the logger before use
///
/// ## Key Differences from `std::dbg!`
///
/// - Returns `()` instead of the original value
/// - Requires explicit logger initialization
///
/// ## Examples
///
/// Basic usage
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
///
/// 1. Uses `core::any::type_name_of_val` for type information
/// 2. Formats output as: `{variable_name}: {type} = {debug_representation}`
/// 3. Multiple arguments generate separate log entries
#[macro_export]
macro_rules! dbg_ref {
  ($val:expr $(,)?) => {{
    match &$val {
      tmp => {
        log::debug!(
          "{name}: {type_name} = {tmp:#?}",
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
// ===========================

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
    ($($crate::dbg!($val)),+,)
  };
}
