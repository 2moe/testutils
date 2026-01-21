use tinyvec::TinyVec;
pub type StrVec<'a, const N: usize> = TinyVec<[&'a str; N]>;
pub type CowStrVec<'a, const N: usize> = TinyVec<[alloc::borrow::Cow<'a, str>; N]>;
pub type MiniStrVec<const N: usize> = TinyVec<[crate::os_cmd::MiniStr; N]>;

pub mod cargo_build;
mod cargo_doc;
mod cargo_fmt;

pub use cargo_build::CargoCmd;
pub use cargo_doc::CargoDoc;
pub use cargo_fmt::CargoFmt;
