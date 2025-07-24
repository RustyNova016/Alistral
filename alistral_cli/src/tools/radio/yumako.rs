use core::fmt::Display;
use core::ops::Deref as _;
use std::collections::HashMap;

use alistral_core::cli::colors::AlistralColors;
use chrono::DateTime;
use chrono::Utc;
use clap::Parser;
use clap::ValueEnum;
use interzic::models::playlist_stub::PlaylistStub;
use interzic::models::services::listenbrainz::Listenbrainz;
use interzic::models::services::youtube::Youtube;
use itertools::Itertools;
use tracing::debug;
use tracing::error;
use tuillez::fatal_error::FatalError;
use tuillez::fatal_error::IntoFatal;
use tuillez::formatter::FormatWithAsync;
use yumako_jams::RadioStream;
use yumako_jams::json::radio::Radio;
use yumako_jams::radio_item::RadioItem;
use yumako_jams::radio_stream::RadioStreamaExt;
use yumako_jams::radio_variables::RadioVariables;

use crate::ALISTRAL_CLIENT;
use crate::models::cli::parsers::yumako_parser::parse_yumako_variables;
use crate::models::config::Config;
use crate::models::config::config_trait::ConfigFile as _;
use crate::models::config::recording_timeout::RecordingTimeoutConfig;
use crate::models::data_storage::DataStorage;
use crate::tools::radio::convert_recordings;
use crate::utils::constants::LISTENBRAINZ_FMT;
use crate::utils::data_file::DataFile as _;

#[derive(Parser, Debug, Clone)]
pub struct RadioYumakoCommand {
    /// The name of the radio to inspect
    radio_name: String,

    /// Name of the user to send the playlist to.
    #[arg(short, long)]
    username: Option<String>,

    /// Your user token.
    ///
    /// You can find it at <https://listenbrainz.org/settings/>.
    /// If it's set in the config file, you can ignore this argument
    #[arg(short, long)]
    token: Option<String>,

    /// Where to output the radio
    #[arg(short, long, default_value_t = RadioOutput::Listenbrainz)]
    output: RadioOutput,

    /// Radio arguments
    arguments: Vec<String>,
}

impl RadioYumakoCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        let username = Config::check_username(&self.username);
        let token = Config::check_token(&username, &self.token);

        let radio_schema = self.get_radio_schema()?;
        let args = self.get_radio_arguments(&username)?;

        debug!("Compiling radio");
        let radio = match radio_schema
            .clone()
            .to_stream(&ALISTRAL_CLIENT.yumako_jams, args.clone())
        {
            Ok(val) => val,
            Err(err) => {
                compilation_error(err);
                return Ok(());
            }
        };
        debug!("Compiled radio");

        let radio_items = Self::collect_radio(radio, args).await?;

        self.output_radio(conn, radio_schema, radio_items, username, &token)
            .await
            .expect_fatal("Couldn't send the playlist")?;

        Ok(())
    }

    /// Set up the radio arguments from the user inputs and config
    fn get_radio_arguments(&self, username: &str) -> Result<RadioVariables, crate::Error> {
        let mut args = parse_yumako_variables(&self.arguments.join(" "))?;

        args.entry("username".to_string()).or_insert(
            serde_json::to_value(username)
                .expect_fatal("Couldn't read the username as a json Value")?,
        );

        args.insert(
            "timeouts".to_string(),
            serde_json::to_value(load_timeouts())
                .expect_fatal("Couldn't read the timeouts as a json Value")?,
        );

        // Wrap the args
        Ok(RadioVariables::new(args))
    }

    /// Find the radio schema
    fn get_radio_schema(&self) -> Result<Radio, crate::Error> {
        let radio_schema =
            Radio::from_file(format!("./yumako_jams/exemples/{}.json", self.radio_name))
                .expect_fatal("Couldn't read the radio")?;
        Ok(radio_schema)
    }

    async fn collect_radio(
        radio: RadioStream<'_>,
        args: RadioVariables,
    ) -> Result<Vec<RadioItem>, crate::Error> {
        let radio = radio.collect_with_args(args)?.await;
        let mut error_count = 0;
        let mut clean_tracks = Vec::new();

        for track in radio {
            match track {
                Ok(track) => clean_tracks.push(track),
                Err(err) => {
                    error!("Encountered Error on playlist generation: {}", err);

                    if error_count > 5 {
                        FatalError::new_string("Found too many errors. Aborting").panic();
                    }

                    error_count += 1;
                }
            }
        }

        Ok(clean_tracks)
    }

    /// Output the radio based on the format associated
    async fn output_radio(
        &self,
        conn: &mut sqlx::SqliteConnection,
        radio_schema: Radio,
        radio_items: Vec<RadioItem>,
        username: String,
        token: &str,
    ) -> Result<(), crate::Error> {
        match self.output {
            RadioOutput::List => {
                print_radio(&radio_items).await?;
            }
            RadioOutput::Listenbrainz => {
                Listenbrainz::create_playlist(
                    &ALISTRAL_CLIENT.interzic,
                    Self::radio_to_playlist(conn, radio_schema, radio_items).await?,
                    username,
                    token,
                )
                .await?;
            }
            RadioOutput::Youtube => {
                let _playlist_id = Youtube::create_playlist(
                    &ALISTRAL_CLIENT.interzic,
                    Self::radio_to_playlist(conn, radio_schema, radio_items).await?,
                    Some(username),
                )
                .await?;
            }
        };

        Ok(())
    }

    async fn radio_to_playlist(
        conn: &mut sqlx::SqliteConnection,
        radio_schema: Radio,
        radio_items: Vec<RadioItem>,
    ) -> Result<PlaylistStub, crate::Error> {
        let counter = DataStorage::load().expect_fatal("Couldn't load data storage")?;

        Ok(PlaylistStub {
            title: format!(
                "Radio: {} #{}",
                radio_schema.name,
                counter.write().unwrap().incr_playlist_count()
            ),
            description: format!(
                "{}\n\nAutomatically generated by: https://github.com/RustyNova016/Alistral",
                radio_schema.description
            ),
            recordings: convert_recordings(
                conn,
                radio_items
                    .into_iter()
                    .map(|t| t.entity().clone())
                    .collect_vec(),
            )
            .await
            .expect_fatal("Couldn't convert recordings for playlist")?,
        })
    }
}

/// Read the recording timeouts
pub fn load_timeouts() -> HashMap<String, DateTime<Utc>> {
    let config = RecordingTimeoutConfig::load().expect("Couldn't fetch the timeout config");
    let config = config.read_or_panic();
    config.deref().deref().clone()
}

pub fn compilation_error(err: yumako_jams::Error) {
    println!();
    println!(
        "{}",
        "Radio compilation error".as_color_title((225, 125, 0))
    );
    println!();
    println!("{err}");
    println!();
    // #[cfg(debug_assertions)]
    // println!("{err:#?}")
}

async fn print_radio(
    radio: &Vec<RadioItem>,
) -> Result<(), crate::Error> {
    for track in radio {
        println!(
            "[{}] {}",
            track.score.round_sf(3).unwrap(),
            track.entity().format_with_async(&LISTENBRAINZ_FMT).await?
        )
    }

    Ok(())
}

#[derive(ValueEnum, Clone, Debug)]
pub enum RadioOutput {
    List,
    Listenbrainz,
    Youtube,
}

impl Display for RadioOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List => write!(f, "list"),
            Self::Listenbrainz => write!(f, "listenbrainz"),
            Self::Youtube => write!(f, "youtube"),
        }
    }
}
