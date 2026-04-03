use crate::interface::errors::no_panic_err::friendly_panic;
use crate::tools::radio::yumako::RadioYumakoCommand;
use crate::utils::constants::paths::YUMAKO_VARS_DIR;

impl RadioYumakoCommand {
    /// Read a variables file and returns its data
    pub fn read_variable_file(&self) -> Result<(), crate::Error> {
        let path = YUMAKO_VARS_DIR.join(self.var_file.clone().unwrap_or_default());

        if !path.is_file() {
            friendly_panic(
                "Incorrect path",
                &format!(
                    "The path `{}` doesn't exist / isn't a file. Make sure that it exists",
                    path.display()
                ),
            )?
        }

        Ok(())
    }
}
