use std::str::FromStr;

use alistral_core::database::fetching::listens::ListenFetchQuery;
use alistral_core::database::fetching::listens::ListenFetchQueryReturn;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use chrono::Duration;
use chrono::Utc;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use rust_decimal::Decimal;
use tracing::info;
use tuillez::extensions::chrono_exts::DurationExt as _;
use tuillez::formatter::FormatWithAsync;

use crate::ALISTRAL_CLIENT;
use crate::models::cli::BumpCLI;
use crate::models::config::Config;
use crate::utils::cli::read_mbid_from_input;
use crate::utils::constants::LISTENBRAINZ_FMT;

pub async fn bump_command(conn: &mut sqlx::SqliteConnection, bump: BumpCLI) {
    let username = Config::check_username(&bump.username);

    let recording = match bump.recording {
        Some(val) => {
            let mbid = read_mbid_from_input(&val).expect("Couldn't parse MBID");

            Recording::get_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db, &mbid)
                .await
                .expect("Couldn't get the recording")
                .expect("The latest listen isn't mapped. Canceling")
        }
        None => {
            let listens = ListenFetchQuery::builder()
                .fetch_recordings_redirects(false)
                .returns(ListenFetchQueryReturn::Mapped)
                .user(username.to_string())
                .build()
                .fetch(conn, &ALISTRAL_CLIENT.core)
                .await
                .expect("Couldn't fetch the new listens");

            listens
                .get_latest_listen()
                .expect("No listens were found")
                .get_recording_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db)
                .await
                .expect("Couldn't fetch recording")
                .expect("The latest listen isn't mapped. Canceling")
        }
    };

    let multiplier = Decimal::from_str(&bump.multiplier.unwrap_or_else(|| "1.1".to_string()))
        .expect("Couldn't parse the multiplier");

    let duration = match bump.duration {
        Some(dur) => Duration::from_human_string(&dur).expect("Couldn't parse the duration."),
        None => Duration::from_human_string("3 months").expect("Couldn't parse the duration."),
    };

    let conf = Config::load_or_panic();

    info!(
        "Adding bump to {}, giving a {} multiplier for {}",
        recording
            .format_with_async(&LISTENBRAINZ_FMT)
            .await
            .expect("Error while getting recording credits"),
        multiplier,
        duration.to_humantime().unwrap()
    );

    conf.write_or_panic().bumps.add_bump(
        recording.mbid.clone(),
        username,
        multiplier,
        Utc::now() + duration,
    );
}

pub async fn bump_down_command(conn: &mut sqlx::SqliteConnection, mut bump: BumpCLI) {
    bump.multiplier = bump.multiplier.or_else(|| Some("0.9".to_string()));
    bump_command(conn, bump).await;
}
