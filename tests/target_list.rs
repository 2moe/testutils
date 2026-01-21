// rustc --print target-list | awk '{gsub(/-|\./, "_", $0); printf("%s,",$0) }'
use std::{io, process::Command};

use tap::Pipe;
use testutils::{
  get_pkg_name,
  os_cmd::{self, RunnableCommand},
};

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  r#"
    rustc
    --print target-list
  "#
  .pipe(os_cmd::Runner::from)
  .run()?;

  Ok(())
}
