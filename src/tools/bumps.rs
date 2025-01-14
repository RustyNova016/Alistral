use std::str::FromStr;

use chrono::Duration;
use chrono::Utc;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use rust_decimal::Decimal;

use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::models::cli::BumpCLI;
use crate::models::config::Config;
use crate::utils::cli::display::RecordingExt as _;
use crate::utils::cli::read_mbid_from_input;
use crate::utils::extensions::chrono_ext::DurationExt as _;
use crate::utils::println_cli;

pub async fn bump_command(conn: &mut sqlx::SqliteConnection, bump: BumpCLI) {
    let username = Config::check_username(&bump.username);

    let recording = match bump.recording {
        Some(val) => {
            let mbid = read_mbid_from_input(&val).expect("Couldn't parse MBID");

            Recording::get_or_fetch(conn, &mbid)
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
                .fetch(conn)
                .await
                .expect("Couldn't fetch the new listens");

            listens
                .get_latest_listen()
                .expect("No listens were found")
                .get_recording_or_fetch(conn)
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

    println_cli(format!(
        "Adding bump to {}, giving a {} multiplier for {}",
        recording
            .pretty_format_with_credits(conn, true)
            .await
            .expect("Error while getting recording credits"),
        multiplier,
        duration.to_humantime().unwrap()
    ));

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
