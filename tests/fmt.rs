use std::io;

use testutils::os_cmd::{RunnableCommand, presets::CargoFmt};

#[ignore]
#[test]
fn fmt() -> io::Result<()> {
  CargoFmt::default().run()
}
