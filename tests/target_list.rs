// rustc --print target-list | awk '{gsub(/-|\./, "_", $0); printf("%s,",$0) }'
use std::io;

use tap::Pipe;
use testutils::os_cmd::Runner;

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  r#"
    rustc
    --print target-list
  "#
  .pipe(Runner::from)
  .run_command();

  Ok(())
}
