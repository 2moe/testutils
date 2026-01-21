//! ```ignore, sh
//! cargo open-doc
//! ```
use std::io;

use testutils::{
  get_pkg_name,
  os_cmd::{RunnableCommand, presets::CargoDoc},
};

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  CargoDoc::default()
    .with_pkg(get_pkg_name!())
    .with_open(false)
    .run()
}
