//! ```ignore, sh
//! cargo open-doc
//! ```
use std::io;

use testutils::{
  get_pkg_name,
  os_cmd::{RunnableCommand, Runner, presets::CargoDoc},
  tap::{Pipe, Tap},
};

#[ignore]
#[test]
fn build_doc() -> io::Result<()> {
  let build = |pkg| {
    CargoDoc::default()
      .with_pkg(pkg)
      .with_enable_private_items(false)
      // .with_all_features(false)
      .with_open(false) // Disable to maintain compatibility with remote SSH.
      .run()
  };

  get_pkg_name!().pipe(build)
}

/// Uses `miniserve` instead of `.with_open(true)` on CargoDoc to ensure
/// compatibility with Remote-SSH.
#[ignore]
#[test]
fn serve_doc() -> io::Result<()> {
  build_doc()?;

  let dir = env!("CARGO_MANIFEST_DIR");
  let pkg = get_pkg_name!();

  format!(
    "miniserve
    {dir:?}/target/doc
    --index {pkg}/index.html
    ",
  )
  .pipe_deref(Runner::from)
  .tap(|_| eprintln!("http://127.0.0.1:8080/{pkg}/index.html"))
  .run()
  .inspect_err(|e| eprintln!("{e:?};\n cargo binstall miniserve"))
}
