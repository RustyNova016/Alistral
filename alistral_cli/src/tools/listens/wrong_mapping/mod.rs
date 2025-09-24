use alistral_core::database::fetching::listens::ListenFetchQuery;
use alistral_core::database::fetching::listens::ListenFetchQueryReturn;
use display::display_wrong_mapping;
use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;
use musicbrainz_db_lite::HasArtistCredits as _;
use strsim::sorensen_dice;

use crate::ALISTRAL_CLIENT;
use crate::models::config::config_trait::ConfigFile as _;
use crate::models::config::whitelisted_wrong_mappings::WhilistedWrongMappings;

pub mod display;

pub async fn wrong_mapping(conn: &mut sqlx::SqliteConnection, username: String) {
    let config = WhilistedWrongMappings::load().expect("Couldn't load whitelisted mappings");
    let listens = ListenFetchQuery::builder()
        .fetch_recordings_redirects(false)
        .returns(ListenFetchQueryReturn::Mapped)
        .user(username.to_string())
        .build()
        .fetch(conn, &ALISTRAL_CLIENT.core)
        .await
        .expect("Couldn't fetch listens");

    for listen in listens.iter().unique_by(|l| &l.recording_msid) {
        let messybrainz_data =
            MessybrainzSubmission::find_by_msid(conn, listen.recording_msid.clone())
                .await
                .expect("Couldn't fetch the messybrainz data of the listen")
                .expect("Couldn't find the messybrainz data of the listen");

        let recording = listen
            .get_recording_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db)
            .await
            .expect("Couldn't fetch recording data")
            .expect("The listen should be mapped");

        if config
            .read_or_panic()
            .is_whitelisted(&messybrainz_data.msid, &recording.mbid)
        {
            continue;
        }

        let formated_messy = format!(
            "{} {}",
            messybrainz_data.artist_credit, messybrainz_data.recording
        )
        .to_lowercase();
        let formated_recording = format!(
            "{} {}",
            recording
                .get_artist_credits_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db)
                .await
                .expect("Couldn't get the artist credit"),
            recording.title
        )
        .to_lowercase();
        let score = sorensen_dice(&formated_messy, &formated_recording);

        if score != 1.0 {
            let continu = display_wrong_mapping(
                conn,
                &mut config.write_or_panic(),
                &messybrainz_data,
                &recording,
                listen,
                &username,
            )
            .await;
            if !continu {
                break;
            }
        }
    }
}
