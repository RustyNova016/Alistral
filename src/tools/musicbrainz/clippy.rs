use std::collections::VecDeque;

use alistral_core::cli::colors::AlistralColors as _;
use color_eyre::owo_colors::OwoColorize as _;
use futures::TryStreamExt;
use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use tracing::info;

use crate::api::clients::ALISTRAL_CLIENT;
use crate::datastructures::clippy::dash_eti::DashETILint;
use crate::datastructures::clippy::missing_release_barcode::MissingBarcodeLint;
use crate::datastructures::clippy::missing_remix_rel::MissingRemixRelLint;
use crate::datastructures::clippy::missing_remixer_rel::MissingRemixerRelLint;
use crate::datastructures::clippy::missing_work::MissingWorkLint;
use crate::datastructures::clippy::soundtrack_without_disambiguation::SoundtrackWithoutDisambiguationLint;
use crate::datastructures::clippy::suspicious_remix::SuspiciousRemixLint;
use crate::models::clippy::MbClippyLint;
use crate::utils::cli::await_next;
use crate::utils::cli::display::MainEntityExt;
use crate::utils::whitelist_blacklist::WhitelistBlacklist;

pub async fn mb_clippy(
    conn: &mut sqlx::SqliteConnection,
    start_mbid: &str,
    new_first: bool,
    filter: &WhitelistBlacklist<String>,
) {
    let start_node = Recording::fetch_and_save(conn, &ALISTRAL_CLIENT.musicbrainz_db, start_mbid)
        .await
        .unwrap()
        .expect("Couldn't find MBID");

    let mut queue = VecDeque::new();
    queue.push_back(MainEntity::Recording(start_node));
    let mut seen = Vec::new();

    while let Some(mut entity) = get_new_element(&mut queue, new_first) {
        if seen
            .iter()
            .any(|done: &MainEntity| done.is_equal_by_mbid(&entity))
        {
            continue;
        }

        entity
            .refetch_and_load(conn, &ALISTRAL_CLIENT.musicbrainz_db)
            .await
            .expect("Couldn't fetch entity");

        check_lint::<DashETILint>(conn, &mut entity, filter).await;
        check_lint::<MissingWorkLint>(conn, &mut entity, filter).await;
        check_lint::<MissingBarcodeLint>(conn, &mut entity, filter).await;
        check_lint::<SuspiciousRemixLint>(conn, &mut entity, filter).await;
        check_lint::<MissingRemixRelLint>(conn, &mut entity, filter).await;
        check_lint::<MissingRemixerRelLint>(conn, &mut entity, filter).await;
        check_lint::<SoundtrackWithoutDisambiguationLint>(conn, &mut entity, filter).await;

        println!(
            "Checked {}",
            entity
                .pretty_format(conn, false)
                .await
                .expect("Error while formating the name of the entity")
        );
        println!();

        get_new_nodes(conn, &entity, &mut queue)
            .await
            .expect("Couldn't get new items to process");

        seen.push(entity);
    }

    println!("No more data to process");
}

fn get_new_element(queue: &mut VecDeque<MainEntity>, new_first: bool) -> Option<MainEntity> {
    if new_first {
        queue.pop_front()
    } else {
        queue.pop_back()
    }
}

async fn check_lint<L: MbClippyLint>(
    conn: &mut sqlx::SqliteConnection,
    entity: &mut MainEntity,
    filter: &WhitelistBlacklist<String>,
) {
    // Check if the lint is allowed
    if !filter.is_allowed(&L::get_name().to_string()) {
        return;
    }

    let Some(lint) = L::check(conn, entity)
        .await
        .expect("Error while processing lint")
    else {
        return;
    };

    println!(
        "{}",
        format!("\n {} ", L::get_name())
            .on_truecolor_tup(lint.get_severity().get_color())
            .black()
            .bold()
    );
    println!();
    println!(
        "{}",
        lint.get_body(conn)
            .await
            .expect("Error while processing lint body")
    );

    // Hints
    let hints = lint
        .get_hints(conn)
        .await
        .expect("Error while processing lint hints");
    if !hints.is_empty() {
        println!();
        for hint in hints {
            println!("{hint}");
        }
    }

    // Links
    println!();
    println!("Links:");
    for link in lint
        .get_links(conn)
        .await
        .expect("Error while processing lint links")
    {
        println!("    - {link}");
    }

    println!();
    await_next();
    entity
        .refetch_and_load(conn, &ALISTRAL_CLIENT.musicbrainz_db)
        .await
        .expect("Couldn't fetch entity");
}

async fn get_new_nodes(
    conn: &mut sqlx::SqliteConnection,
    entity: &MainEntity,
    queue: &mut VecDeque<MainEntity>,
) -> Result<(), crate::Error> {
    info!("Getting new data...");

    match entity {
        MainEntity::Recording(val) => {
            let artists = val
                .get_artists_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db)
                .await?;
            for artist in artists {
                queue.push_front(MainEntity::Artist(artist));
            }

            let releases = val
                .get_releases_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db)
                .await?;
            for release in releases {
                queue.push_front(MainEntity::Release(release));
            }

            let works = val
                .get_works_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db)
                .await?;
            for work in works {
                queue.push_front(MainEntity::Work(work));
            }
        }
        MainEntity::Release(val) => {
            let recordings = val
                .get_recordings_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db)
                .await?;
            for recording in recordings {
                queue.push_front(MainEntity::Recording(recording));
            }
        }
        MainEntity::Artist(val) => {
            let recordings: Vec<Recording> = val
                .browse_or_fetch_artist_recordings(conn, &ALISTRAL_CLIENT.musicbrainz_db)
                .try_collect()
                .await?;
            for recording in recordings {
                queue.push_front(MainEntity::Recording(recording));
            }
        }
        _ => {}
    }

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use crate::tools::musicbrainz::clippy::mb_clippy;
//     use crate::utils::whitelist_blacklist::WhitelistBlacklist;

//     #[tokio::test]
//     async fn mb_clippy_test() {
//         mb_clippy(
//             "b67fae1f-3037-4c01-bff9-b5e877220267",
//             false,
//             &WhitelistBlacklist::default(),
//         )
//         .await;
//     }
// }
