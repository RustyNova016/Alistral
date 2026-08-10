use crate::tools::docs::config_schema::DocsConfigSchemaCommand;

pub mod config_schema;

/// Commands to get documentation
#[derive(clap::Parser, Debug, Clone)]
#[command(version, about)]
pub struct DocsCommand {
    #[command(subcommand)]
    pub command: DocsSubcommands,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum DocsSubcommands {
    ConfigSchema(DocsConfigSchemaCommand),
}

impl DocsCommand {
    pub async fn run(&self) {
        match &self.command {
            DocsSubcommands::ConfigSchema(val) => val.run(),
        }
    }
}
