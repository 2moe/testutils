#[cfg(feature = "std")]
pub mod normal;
pub use normal::dbg;

#[cfg(feature = "std")]
pub mod buf_lock;

mod macros;
