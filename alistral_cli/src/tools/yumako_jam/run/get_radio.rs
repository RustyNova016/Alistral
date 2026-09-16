use std::fs::File;
use std::io::Read as _;
use std::path::PathBuf;

use snafu::ResultExt as _;
use yumako_jams::models::radio_file::radio::Radio;

use crate::tools::yumako_jam::run::error::RadioFileParseSnafu;
use crate::tools::yumako_jam::run::error::RadioFileReadSnafu;
use crate::tools::yumako_jam::run::error::RadioNotFoundSnafu;
use crate::tools::yumako_jam::run::error::YumakoRunCommandError;
use crate::utils::constants::paths::YUMAKO_DIR;

pub(super) fn get_radio(name: &str) -> Result<Radio, YumakoRunCommandError> {
    let path = get_radio_path(name)?;

    let mut file = File::open(&path).context(RadioFileReadSnafu)?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .context(RadioFileReadSnafu)?;

    Radio::from_file_content(&content).context(RadioFileParseSnafu)
}

pub fn get_radio_path(name: &str) -> Result<PathBuf, YumakoRunCommandError> {
    let path = YUMAKO_DIR.join(format!("{name}.json5"));
    if path.exists() {
        return Ok(path);
    }

    let path = YUMAKO_DIR.join(format!("{name}.json"));
    if path.exists() {
        return Ok(path);
    }

    RadioNotFoundSnafu { path }.fail()
}
