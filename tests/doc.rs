use std::io;

use tap::Pipe;
use testutils::os_cmd::{Runner, presets::CargoDoc};

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  CargoDoc::default()
    .with_enable_private_items(false)
    .pipe(Runner::from)
    .run()
}
