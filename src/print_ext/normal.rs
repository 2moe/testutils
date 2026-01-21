/*!
Lightweight stdio printing helpers.

This module provides small convenience wrappers around the standard
`print!`/`println!` and `eprint!`/`eprintln!` macros, plus debug variants.

The goal is to keep call sites terse and consistent when emitting text to
stdout/stderr.
*/

use core::fmt::{Debug, Display};

/// Prints a debug representation of `msg` to **stdout**, followed by a newline.
///
/// This is a lightweight convenience wrapper around `println!("{msg:?}")`.
/// Unlike `std::dbg!`, it does **not** print file/line info and does **not**
/// return the value.
///
/// Accepts dynamically-sized types (`str`, slices, trait objects) via `?Sized`.
///
/// => {msg:?} |> println!
#[inline]
pub fn dbg<T: Debug + ?Sized>(msg: &T) {
  println!("{msg:?}")
}

#[inline]
/// => msg |> println!
pub fn puts<T: Display + ?Sized>(msg: &T) {
  println!("{msg}")
}

#[inline]
/// => msg |> print!
pub fn print<T: Display + ?Sized>(msg: &T) {
  print!("{msg}")
}

#[inline]
/// => {msg:?} |> eprintln!
pub fn edbg<T: Debug + ?Sized>(msg: &T) {
  eprintln!("{msg:?}")
}

#[inline]
/// => msg |> eprintln!
pub fn eputs<T: Display + ?Sized>(msg: &T) {
  eprintln!("{msg}")
}

#[inline]
/// => msg |> eprint!
pub fn eprint<T: Display + ?Sized>(msg: &T) {
  eprint!("{msg}")
}
