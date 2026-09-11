use std::fs::File;
use std::path::PathBuf;

use snafu::ResultExt;
use yumako_jams::radio_variables::RadioInputs;

use crate::tools::radio::yumako::errors::InputFileSnafu;
use crate::tools::radio::yumako::errors::YumakoCommandError;
use crate::tools::radio::yumako::inputs;

pub(super) fn get_radio_inputs(input_files: &[PathBuf], input_jsons: &[String]) -> Result<RadioInputs, YumakoCommandError>{
    let mut inputs = RadioInputs::default();

    for input_file in input_files {
        let file = File::open(input_file).context(InputFileSnafu {path: input_file});
    }

    Ok(inputs)
}