/// `env!("CARGO_PKG_NAME")`
#[macro_export]
macro_rules! get_pkg_name {
  () => {
    env!("CARGO_PKG_NAME")
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
/// Generates a **static** `OnceLock` variable with the given name and type.
///
/// ## Example
///
/// ```
/// use testutils::new_once_lock;
///
/// fn static_a<'s>(n: u8) -> &'s u8 {
///   new_once_lock!(A: u8); // => static A: OnceLock<u8> = OnceLock::new();
///   A.get_or_init(|| n)
/// }
/// assert_eq!(static_a(3), &3);
/// assert_eq!(static_a(5), &3);
/// assert_eq!(static_a(42), &3);
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! new_once_lock {
  ($name:ident : $t:ty) => {
    static $name: ::std::sync::OnceLock<$t> = ::std::sync::OnceLock::new();
  };
}
