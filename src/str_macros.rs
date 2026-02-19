pub use const_str;
// https://doc.rust-lang.org/cargo/reference/environment-variables.html

/// `env!("CARGO_PKG_NAME")`
#[macro_export]
macro_rules! get_pkg_name {
  () => {
    env!("CARGO_PKG_NAME")
  };
}

/// Example:
///   const_upper_case!(xdg_data_home)
/// expands to:
///   "XDG_DATA_HOME"
#[macro_export]
#[cfg(feature = "const_str")]
macro_rules! const_upper_case {
  ($s:expr) => {
    $crate::str_macros::const_str::convert_ascii_case!(upper, $s)
  };
}

/// Example:
///   const_lower_case!(XDG_DATA_HOME)
/// expands to:
///   "xdg_data_home"
#[macro_export]
#[cfg(feature = "const_str")]
macro_rules! const_lower_case {
  ($s:expr) => {
    $crate::str_macros::const_str::convert_ascii_case!(lower, $s)
  };
}

/// Expands to an uppercase Cargo cfg environment variable name.
///
/// Example:
///   cargo_cfg!(target_env)
/// expands to:
///   std::env::var("CARGO_CFG_TARGET_ENV")
#[macro_export]
#[cfg(feature = "const_str")]
#[cfg(feature = "std")]
macro_rules! cargo_cfg {
  ($name:ident) => {{
    let env_name: &str = $crate::str_macros::const_str::convert_ascii_case! {
      upper,
      concat!["cargo_cfg_", stringify!($name)]
    };
    ::std::env::var(env_name)
  }};
}

/// Expands to an uppercase Cargo pkg environment variable name.
///
/// Example:
///   cargo_pkg_str!(version)
/// expands to:
///   "CARGO_PKG_VERSION"
#[macro_export]
#[cfg(feature = "const_str")]
macro_rules! cargo_pkg_str {
  ($name:ident) => {
    $crate::str_macros::const_str::convert_ascii_case! {
      upper,
      concat!["cargo_pkg_", stringify!($name)]
    }
  };
}
