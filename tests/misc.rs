use testutils::{dbg_ref, os_cmd::presets::CargoFmt};

pub(crate) fn init_logger() {
  env_logger::builder()
    .filter_level(log::LevelFilter::Debug)
    .init()
}

#[test]
#[ignore]
fn test_macro() {
  dbg!(testutils::get_pkg_name!());
}

#[test]
fn test_cargo_fmt_cmd() {
  let cmd = CargoFmt::default().with_nightly(false);
  // dbg!(&cmd);
  assert!(!cmd.get_nightly());
}

#[ignore]
#[test]
fn test_dbg_ref_macro() {
  use testutils::dbg_ref;
  init_logger();

  log::info!("OK");
  log::warn!("E");

  let x = 42;
  dbg_ref!(x); // Prints: [DEBUG] x: i32 = 42

  let y = "hello";
  dbg_ref!(y); // Prints: [DEBUG] y: &str = "hello"

  let z = vec![1, 2, 3];
  dbg_ref!(z); // Prints: [DEBUG] z: alloc::vec::Vec<i32> = [1, 2, 3]

  let a = 10;
  let b = true;
  dbg_ref!(a, b);
  // Prints:
  // [DEBUG] a: i32 = 10
  // [DEBUG] b: bool = true
}

#[ignore]
#[test]
fn test_dbg_macro() {
  use testutils::dbg;

  let x = 42;
  dbg!(x); // Prints: x: i32 = 42

  let y = "hello";
  dbg!(y); // Prints: y: &str = "hello"

  let z = vec![1, 2, 3];
  dbg!(z); // Prints: z: alloc::vec::Vec<i32> = [1, 2, 3]

  let a = 10;
  let b = true;
  dbg!(a, b);
  // Prints:
  //  a: i32 = 10
  //  b: bool = true
}
