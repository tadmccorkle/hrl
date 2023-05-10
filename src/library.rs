use std::{fs, path::Path};

use super::{Error, Result};

pub mod auto;
pub mod manual;

const HRL_EXT_A: &str = "hrla";
const HRL_EXT_B: &str = "hrlb";

fn remove_hrl_files(path: &Path) -> Result<()> {
    let hrla_res = remove_file(&path.with_extension(HRL_EXT_A));
    let hrlb_res = remove_file(&path.with_extension(HRL_EXT_B));

    hrla_res.and(hrlb_res)
}

fn remove_file(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_file(path).map_err(|e| Error::IoFailure(e))
    } else {
        Ok(())
    }
}
