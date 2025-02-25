use std::io;

use tap::Pipe;
use testutils::{
  get_pkg_name,
  os_cmd::{Runner, presets::CargoDoc},
};

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  CargoDoc::default()
    .with_pkg(get_pkg_name!())
    .pipe(Runner::from)
    .run()
}
