use std::io;

use tap::Pipe;
use testutils::os_cmd::{Runner, presets::CargoFmt};

#[ignore]
#[test]
fn fmt() -> io::Result<()> {
  CargoFmt::default()
    .pipe(Runner::from)
    .run()
}
