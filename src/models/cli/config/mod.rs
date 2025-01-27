use crate::models::config::config_trait::ConfigFile as _;
use crate::models::config::recording_timeout::RecordingTimeoutConfig;
use crate::models::config::Config;
use crate::utils::cli::read_mbid_from_input;
use crate::utils::extensions::chrono_ext::DurationExt;
use chrono::Duration;
use clap::Parser;
use clap::Subcommand;
use listen_config::ListenConfigCli;

pub mod listen_config;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct ConfigCli {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCommands {
    /// Prevent an MSID to appear in the mbid mapper.
    BlacklistMapperMSID {
        /// The msid to blacklist
        msid: String,

        /// Remove it from the blacklist
        #[arg(long, action)]
        remove: bool,
    },

    /// Associate an user token to an username. This makes `--token` arguments optional, and prevent always having to insert it
    SetToken {
        /// Name of the user to add the token
        username: String,

        /// User token
        token: String,
    },

    /// Prevent the recording to appear on radios for a while. If you're burn out of a track and need it gone, use this.
    Timeout {
        /// A string containing a MBID of a recording
        recording: String,

        /// A duration to timeout for
        duration: String,
    },

    /// Configuration targeting listen data
    Listens(ListenConfigCli),

    /// Set the default username
    DefaultUser { username: String },
}

impl ConfigCommands {
    pub async fn run(&self) -> color_eyre::Result<()> {
        match self {
            Self::SetToken { username, token } => {
                let conf = Config::load()?;
                conf.write_or_panic()
                    .set_token(username.clone(), token.clone());
            }

            Self::Timeout {
                recording,
                duration,
            } => {
                let id = read_mbid_from_input(recording).expect("Couldn't parse MBID");
                let config_guard = RecordingTimeoutConfig::load()?;
                let mut config = config_guard.write_or_panic();
                config.set_timeout(&id, Duration::from_human_string(duration)?);
            }

            Self::BlacklistMapperMSID { msid, remove } => {
                let conf = Config::load()?;
                if !remove {
                    conf.write_or_panic()
                        .add_blacklisted_msid(msid.to_string())?;
                } else {
                    conf.write_or_panic().remove_blacklisted_msid(msid)?;
                }
            }
            Self::Listens(val) => val.run().await?,

            Self::DefaultUser { username } => {
                let conf = Config::load()?;
                conf.write_or_panic().default_user = Some(username.clone());
            }
        }

        Ok(())
    }
}
