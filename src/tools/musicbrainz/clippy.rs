use std::collections::VecDeque;

use alistral_core::cli::colors::AlistralColors as _;
use clap::Parser;
use color_eyre::owo_colors::OwoColorize as _;
use futures::TryStreamExt;
use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use symphonize::clippy::clippy_lint::MbClippyLint;
use symphonize::clippy::lints::missing_release_barcode::MissingBarcodeLint;
use symphonize::clippy::lints::missing_remix_rel::MissingRemixRelLint;
use symphonize::clippy::lints::missing_remixer_rel::MissingRemixerRelLint;
use symphonize::clippy::lints::missing_work::MissingWorkLint;
use symphonize::clippy::lints::soundtrack_without_disambiguation::SoundtrackWithoutDisambiguationLint;
use symphonize::clippy::lints::suspicious_remix::SuspiciousRemixLint;
use tracing::info;
use tuillez::formatter::FormatWithAsync;

use crate::ALISTRAL_CLIENT;
use crate::utils::cli::await_next;
use crate::utils::cli::read_mbid_from_input;
use crate::utils::constants::MUSIBRAINZ_FMT;
use crate::utils::whitelist_blacklist::WhitelistBlacklist;

#[derive(Parser, Debug, Clone)]
/// Search for potential mistakes, missing data and style issues. This allows to quickly pin down errors that can be corrected
///
/// ⚠️ All tips are suggestions. Take them with a grain of salt. If you are unsure, it's preferable to skip.
pub struct MusicbrainzClippyCommand {
    /// The MBID of a recording to start from
    pub start_mbid: Option<String>,

    /// Whether to check FILO (first in, last out) instead of FIFO (first in, first out)
    #[arg(short, long)]
    pub new_first: bool,

    /// List of lints that should only be checked (Note: Put this argument last or before another argument)
    #[arg(short, long, num_args = 0..)]
    pub whitelist: Option<Vec<String>>,

    /// List of lints that should not be checked (Note: Put this argument last or before another argument)
    #[arg(short, long, num_args = 0..)]
    pub blacklist: Option<Vec<String>>,
}

impl MusicbrainzClippyCommand {
    pub async fn run(&self) {
        let mbid = self
            .start_mbid
            .clone()
            .unwrap_or_else(|| "8f3471b5-7e6a-48da-86a9-c1c07a0f47ae".to_string());

        let filter = if let Some(whitelist) = self.whitelist.clone() {
            WhitelistBlacklist::WhiteList(whitelist.clone())
        } else if let Some(blacklist) = self.blacklist.clone() {
            WhitelistBlacklist::BlackList(blacklist.clone())
        } else {
            WhitelistBlacklist::BlackList(Vec::new())
        };

        mb_clippy(
            &read_mbid_from_input(&mbid).expect("Couldn't read mbid"),
            self.new_first,
            &filter,
        )
        .await;
    }
}

pub async fn mb_clippy(start_mbid: &str, new_first: bool, filter: &WhitelistBlacklist<String>) {
    let conn = &mut ALISTRAL_CLIENT
        .musicbrainz_db
        .get_raw_connection()
        .await
        .expect("Couldn't acquire a connection");

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

        check_lint::<MissingWorkLint>(conn, &mut entity, filter).await;
        check_lint::<MissingBarcodeLint>(conn, &mut entity, filter).await;
        check_lint::<SuspiciousRemixLint>(conn, &mut entity, filter).await;
        check_lint::<MissingRemixRelLint>(conn, &mut entity, filter).await;
        check_lint::<MissingRemixerRelLint>(conn, &mut entity, filter).await;
        check_lint::<SoundtrackWithoutDisambiguationLint>(conn, &mut entity, filter).await;

        println!(
            "Checked {}",
            entity
                .format_with_async(&MUSIBRAINZ_FMT)
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

    let Some(lint) = L::check(&ALISTRAL_CLIENT.symphonize, entity)
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
        lint.get_body(&ALISTRAL_CLIENT.symphonize)
            .await
            .expect("Error while processing lint body")
    );

    // Hints
    let hints = lint
        .get_hints(&ALISTRAL_CLIENT.symphonize)
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
        .get_links(&ALISTRAL_CLIENT.symphonize)
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
                .browse_or_fetch_artist_recordings(conn, ALISTRAL_CLIENT.musicbrainz_db.clone())
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
