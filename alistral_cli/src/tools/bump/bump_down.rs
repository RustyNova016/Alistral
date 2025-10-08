use clap::Parser;

use crate::tools::bump::BumpCommand;

/// bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target.
///
/// bump-down is an alias for `bump <RECORDING> <DURATION> 0.9`
///
/// All the bumps are added multiplicatively, so a recording won't disapear. Use the blacklist to remove them.
#[derive(Parser, Debug, Clone)]
pub struct BumpDownCommand {
    /// The recording to bump
    pub recording: Option<String>,

    /// The duration the bump last for (Default: 3 months)
    #[arg(short, long)]
    pub duration: Option<String>,

    /// The multiplier added to the score (Default: 1.1)
    #[arg(short, long)]
    pub multiplier: Option<String>,

    #[arg(short, long)]
    pub username: Option<String>,
}

impl BumpDownCommand {
    pub async fn run(&self) {
        let cmd = BumpCommand {
            duration: self.duration.clone(),
            multiplier: self.multiplier.clone().or_else(|| Some("0.9".to_string())),
            recording: self.recording.clone(),
            username: self.username.clone(),
        };

        cmd.run().await
    }
}
