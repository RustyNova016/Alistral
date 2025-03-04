use std::collections::HashMap;

use chrono::Utc;
use clap::Parser;
use listenbrainz::raw::request::Payload;
use listenbrainz::raw::request::SubmitListens;
use listenbrainz::raw::request::TrackMetadata;
use tuillez::fatal_error::IntoFatal;

use crate::api::clients::ALISTRAL_CLIENT;
use crate::models::config::Config;

#[derive(Parser, Debug, Clone)]
pub struct ListenSubmitCommand {
    /// The name of the recording listened to
    pub recording: String,

    /// The artist credits of the listen
    pub artist_credits: String,

    /// The release of the recording
    #[arg(short = 'g', long)]
    pub release: Option<String>,

    /// Submit the listen on this user
    #[arg(short, long)]
    pub user: Option<String>,

    /// The token of the user
    #[arg(short, long)]
    pub token: Option<String>,
}

impl ListenSubmitCommand {
    pub async fn run(&self) -> Result<(), crate::Error> {
        let mut extras = HashMap::new();
        extras.insert("submission_client", "Alistral (Submit listen)");
        extras.insert("submission_client_version", env!("CARGO_PKG_VERSION"));

        ALISTRAL_CLIENT.listenbrainz.submit_listens(
            &self.get_token(),
            SubmitListens {
                listen_type: listenbrainz::raw::request::ListenType::Single,
                payload: &[Payload {
                    listened_at: Some(Utc::now().timestamp()),
                    track_metadata: TrackMetadata {
                        track_name: self.recording.clone(),
                        artist_name: self.artist_credits.clone(),
                        release_name: self.release.clone(),
                        additional_info: serde_json::to_value(extras)
                            .expect_fatal("Couldn't convert extra data to json")?
                            .as_object()
                            .cloned(),
                    },
                }],
            },
        )?;

        Ok(())
    }

    pub fn get_user(&self) -> String {
        Config::check_username(&self.user)
    }

    pub fn get_token(&self) -> String {
        Config::check_token(&self.get_user(), &self.token)
    }
}
