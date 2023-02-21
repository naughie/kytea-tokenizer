use std::fs::File;
use std::io::Result;
use std::path::Path;
use std::process::Command;

use crate::DELIM_STR;

pub fn run_cmd(
    in_path: impl AsRef<Path>,
    out_path: impl AsRef<Path>,
    model: Option<&str>,
) -> Result<()> {
    kytea_command(model)
        .stdin(File::open(in_path)?)
        .stdout(File::create(out_path)?)
        .output()
        .map(|_| ())
}

pub fn kytea_command(model: Option<&str>) -> Command {
    let mut comm = Command::new("kytea");
    if let Some(model) = model {
        comm.args(["-model", model]);
    }
    comm.args(["-wordbound", DELIM_STR]);
    comm
}
