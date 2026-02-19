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
macro_rules! const_upper_case {
  ($s:expr) => {
    ::const_str::convert_ascii_case!(upper, $s)
  };
}

/// Example:
///   const_lower_case!(XDG_DATA_HOME)
/// expands to:
///   "xdg_data_home"
#[macro_export]
macro_rules! const_lower_case {
  ($s:expr) => {
    ::const_str::convert_ascii_case!(lower, $s)
  };
}

/// Expands to an uppercase Cargo cfg environment variable name.
///
/// Example:
///   cargo_cfg!(target_env)
/// expands to:
///   "CARGO_CFG_TARGET_ENV"
#[macro_export]
macro_rules! cargo_cfg {
  ($name:ident) => {
    ::const_str::convert_ascii_case! {
      upper,
      concat!["cargo_cfg_", stringify!($name)]
    }
  };
}

/// Expands to an uppercase Cargo pkg environment variable name.
///
/// Example:
///   cargo_pkg!(version)
/// expands to:
///   "CARGO_PKG_VERSION"
#[macro_export]
macro_rules! cargo_pkg {
  ($name:ident) => {
    ::const_str::convert_ascii_case! {
      upper,
      concat!["cargo_pkg_", stringify!($name)]
    }
  };
}
