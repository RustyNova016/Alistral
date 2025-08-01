use core::fmt::Display;
use std::fmt::Write;

use alistral_core::datastructures::entity_with_listens::listen_timeframe::ListenTimeframe;
use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::user::UserWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use alistral_core::models::user::UserData;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use clap::Parser;

use musicbrainz_db_lite::FetchAndSave;
use musicbrainz_db_lite::Recording;
use tracing::error;
use tuillez::OwoColorize as _;

use crate::ALISTRAL_CLIENT;
use crate::models::cli::common::Timeframe;
use crate::models::config::Config;
use crate::tools::lookup::components::LookupLine;
use crate::utils::cli::read_mbid_from_input;

#[derive(Parser, Clone, Debug)]
pub(super) struct LookupRecordingCommand {
    /// Get the data of this recording
    recording_mbid: String,

    #[clap(short, long)]
    timeframe: Option<Timeframe>,

    #[clap(short, long)]
    user: Option<String>,
}

impl LookupRecordingCommand {
    pub async fn run(&self) {
        let user = Config::check_username(&self.user);
        let Some(mbid) = read_mbid_from_input(&self.recording_mbid) else {
            error!("Couldn't read the mbid");
            return;
        };

        let Some(recording) =
            Recording::fetch_and_save_as_task(ALISTRAL_CLIENT.musicbrainz_db.clone(), &mbid)
                .await
                .expect("Error while fetching the recording")
        else {
            error!("Couldn't find the requested recording");
            return;
        };

        let userdata = UserData::load_user(&ALISTRAL_CLIENT.core, user)
            .await
            .expect("Couldn't load user data");

        let end = Utc::now();

        let start = match self.timeframe {
            Some(v) => v.get_start_date(),
            None => DateTime::from_timestamp(0, 0).unwrap(),
        };

        let recordings = userdata
            .recordings_with_listens(&ALISTRAL_CLIENT.core)
            .await
            .expect("Couldn't get recording listens");
        let recording = recordings
            .iter()
            .find(|ent| ent.entity().mbid == mbid)
            .cloned()
            .unwrap_or_else(|| RecordingWithListens::new(recording, Default::default()));

        let timeframe = ListenTimeframe::new(start, end, recording);

        if timeframe.all_time().listen_count() == 0 {
            print_empty_report(&timeframe).await;
        } else {
            print_report(&timeframe).await
        }
    }
}

async fn get_title(data: &ListenTimeframe<RecordingWithListens>) -> String {
    let name = data
        .current()
        .entity()
        .format_with_credits(
            &mut *ALISTRAL_CLIENT.mb_conn().await,
            &ALISTRAL_CLIENT.musicbrainz_db,
        )
        .await
        .unwrap();

    if data.previous_opt().is_some() {
        format!(
            " Statistics for {} {}",
            name,
            format!(
                "({} -> {}, compared to {} -> {})",
                data.start().format("%d/%m/%Y"),
                data.end().format("%d/%m/%Y"),
                data.prev_start().format("%d/%m/%Y"),
                data.start().format("%d/%m/%Y"),
            )
            .bright_black()
        )
        .on_green()
        .black()
        .bold()
        .to_string()
    } else {
        format!(" All time statistics for {}", name)
            .on_green()
            .black()
            .bold()
            .to_string()
    }
}

async fn get_listencount(data: &ListenTimeframe<RecordingWithListens>) -> impl Display {
    LookupLine::builder()
        .description("Listen count".to_string())
        .data(data.clone())
        .get_data(|ent| ent.listen_count())
        .value_formater(|a| a.to_string())
        .build()
        .to_string()
        .await
}



async fn print_report(data: &ListenTimeframe<RecordingWithListens>) {
    let mut out = String::new();

    writeln!(out, "{}", get_title(data).await).unwrap();
    writeln!(out).unwrap();
    writeln!(out, "[General]").unwrap();
    writeln!(out, "{}", get_listencount(data).await).unwrap();

    println!("{out}");
}

async fn print_empty_report(data: &ListenTimeframe<RecordingWithListens>) {
    let mut out = String::new();

    writeln!(out, "{}", get_title(data).await).unwrap();
    writeln!(out).unwrap();
    writeln!(out, "This recording hasn't been listened to yet!").unwrap();

    println!("{out}");
}
