use alistral_core::cli::colors::AlistralColors as _;

use crate::utils::yumako_jams::get_radio::YumakoGetRadioError;
use crate::utils::yumako_jams::get_radio::get_radio;

/// Show the info of a radio
#[derive(clap::Parser, Debug, Clone)]
pub struct YumakoInfoCommand {
    /// The name of the radio
    radio_name: String,
}

impl YumakoInfoCommand {
    pub fn run(&self) -> Result<(), YumakoGetRadioError> {
        let radio = get_radio(&self.radio_name)?;

        println!("{}", "[Radio]".yumako_red());
        println!("{}", radio.name);
        println!();
        println!("{}", radio.description);

        if let Some(url) = radio.download_url {
            println!("\nDownload url: {}", url);
        }

        if let Some(val) = radio.version {
            println!("\nVersion: {}", val);
        }

        if let Some(val) = radio.yumako_version {
            println!("\nYumako Jam Version: {}", val);
        }

        println!();
        println!("{}", "[Variables]".yumako_red());

        for variable in radio.inputs {
            if variable.1.hidden {
                continue;
            }

            println!("Name: {}", variable.0);
            println!(
                "Description: {}",
                variable.1.description.as_ref().unwrap_or(&String::new())
            );
            println!(
                "Default: {}",
                serde_json::to_string(&variable.1.default)
                    .expect("serde_json can always convert from Value")
            );
            println!();
        }

        Ok(())
    }
}
