use crate::tiny_container::TinyVec;
pub type TinyCfg<'a, const N: usize> = TinyVec<[&'a str; N]>;
// pub type TinyArgs = TinyVec<[MiniStr; 2]>;
// pub type TinyCowVec<'a, const N: usize> = TinyVec<[alloc::borrow::Cow<'a,
// str>; N]>;

pub mod cargo_build;
mod cargo_doc;
mod cargo_fmt;

pub use cargo_build::CargoCmd;
pub use cargo_doc::CargoDoc;
pub use cargo_fmt::CargoFmt;
