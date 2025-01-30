use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use futures::stream;
use futures::StreamExt;
use interzic::models::playlist_stub::PlaylistStub;
use itertools::Itertools;
use tracing::info;

use crate::api::clients::ALISTRAL_CLIENT;
use crate::api::listenbrainz::global_listen_counts::get_global_listen_counts;
use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::datastructures::radio::collector::RadioCollector;
use crate::datastructures::radio::seeders::listens::ListenSeeder;
use crate::datastructures::radio::sorters::underrated::underrated_sorter;
use crate::models::cli::radio::RadioExportTarget;
use crate::models::data_storage::DataStorage;
use crate::tools::radio::convert_recordings;
use crate::utils::data_file::DataFile as _;

pub async fn underrated_mix(
    conn: &mut sqlx::SqliteConnection,
    seeder: ListenSeeder,
    collector: RadioCollector,
    token: &str,
    target: RadioExportTarget,
) -> color_eyre::Result<()> {
    let username = seeder.username().clone();

    info!("[Seeding] Getting listens");

    // Get the seeder
    let recordings = seeder.seed(conn).await.expect("Couldn't find seed listens");

    // Get the all time listens
    let user_listens = ListenFetchQuery::builder()
        .returns(ListenFetchQueryReturn::Mapped)
        .user(username.to_string())
        .build()
        .fetch(conn)
        .await
        .expect("Couldn't fetch the new listens");

    let user_listens =
        RecordingWithListensCollection::from_listencollection(conn, &ALISTRAL_CLIENT, user_listens)
            .await?;

    // Get the global listen count
    info!("[Seeding] Getting global listen counts");
    let recording_ids = recordings
        .iter_entities()
        .map(|r| r.mbid.to_string())
        .collect_vec();
    let global_listen_counts = get_global_listen_counts(&recording_ids).await?;

    info!("[Sorting] Calculating underated scores");
    let sorted = underrated_sorter(
        recordings.into_iter().collect_vec(),
        &user_listens,
        global_listen_counts,
    );

    info!("[Finalising] Creating radio playlist");
    let collected = collector
        .collect(stream::iter(sorted).map(|r| r.recording().clone()))
        .await;

    info!("[Sending] Sending radio playlist to listenbrainz");
    let counter = DataStorage::load().expect("Couldn't load data storage");
    let playlist = PlaylistStub {
        title: format!(
            "Radio: Underrated recordings #{}",
            counter.write().unwrap().incr_playlist_count()
        ),
        description:  format!("A playlist containing all the tracks that {username} listen to, 
        but seemingly no one else does. Come take a listen if you want to find hidden gems!<br>
        <br>
        The mix is made by calculating a score for each listen. This score is composed of two values:<br>
        - The rank in {username}'s top 1000 recording of all time (First place get 100 points, second get 999.9, etc...)<br>
        - The percentage of the recording's listens being from {username} (Made with this formula: (user listens / worldwide listens) *100)<br>
        <br>
        Made with: https://github.com/RustyNova016/Alistral"
        ),
        recordings: convert_recordings(conn, collected)
            .await
            .expect("Couldn't convert recordings for playlist"),
    };

    target
        .export(playlist, Some(username), Some(token))
        .await
        .expect("Couldn't send the playlist");

    Ok(())
}
