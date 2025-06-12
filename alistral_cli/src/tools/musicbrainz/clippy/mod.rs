use core::fmt::Write as _;
use std::sync::Arc;

use alistral_core::cli::colors::AlistralColors as _;
use clap::Parser;
use futures::StreamExt;
use futures::TryStreamExt;
use futures::pin_mut;
use itertools::Itertools;
use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::main_entities::crawler::crawler;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use streamies::TryStreamies;
use symphonize::clippy::clippy_lint::MbClippyLint;
use symphonize::clippy::lints::dash_eti::DashETILint;
use symphonize::clippy::lints::missing_release_barcode::MissingBarcodeLint;
use symphonize::clippy::lints::missing_remix_rel::MissingRemixRelLint;
use symphonize::clippy::lints::missing_remixer_rel::MissingRemixerRelLint;
use symphonize::clippy::lints::missing_work::MissingWorkLint;
use symphonize::clippy::lints::soundtrack_without_disambiguation::SoundtrackWithoutDisambiguationLint;
use symphonize::clippy::lints::suspicious_remix::SuspiciousRemixLint;
use tracing::debug;
use tuillez::OwoColorize as _;
use tuillez::formatter::FormatWithAsync;

use crate::ALISTRAL_CLIENT;
use crate::database::interfaces::statistics_data::recording_stats;
use crate::models::config::Config;
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

    /// List of lints that should only be checked (Note: Put this argument last or before another argument)
    #[arg(short, long, num_args = 0..)]
    pub whitelist: Option<Vec<String>>,

    /// List of lints that should not be checked (Note: Put this argument last or before another argument)
    #[arg(short, long, num_args = 0..)]
    pub blacklist: Option<Vec<String>>,
}

impl MusicbrainzClippyCommand {
    pub async fn run(&self) {
        let filter = if let Some(whitelist) = self.whitelist.clone() {
            WhitelistBlacklist::WhiteList(whitelist.clone())
        } else if let Some(blacklist) = self.blacklist.clone() {
            WhitelistBlacklist::BlackList(blacklist.clone())
        } else {
            WhitelistBlacklist::BlackList(Vec::new())
        };

        mb_clippy(self.get_start_recordings().await, &filter).await;
    }

    async fn get_start_recordings(&self) -> Vec<Recording> {
        match &self.start_mbid {
            Some(start) => {
                let conn = &mut ALISTRAL_CLIENT
                    .musicbrainz_db
                    .get_raw_connection()
                    .await
                    .expect("Couldn't acquire a connection");
                let start_mbid = read_mbid_from_input(start).expect("Couldn't read mbid");

                let start_node =
                    Recording::fetch_and_save(conn, &ALISTRAL_CLIENT.musicbrainz_db, &start_mbid)
                        .await
                        .unwrap()
                        .expect("Couldn't find MBID");

                vec![start_node]
            }

            None => {
                let recordings = recording_stats(&ALISTRAL_CLIENT, Config::check_username(&None))
                    .await
                    .expect("Couldn't fetch the listened recordings");

                recordings
                    .into_iter()
                    .map(|rec| rec.entity().clone())
                    .collect_vec()
            }
        }
    }
}

pub async fn mb_clippy(start_recordings: Vec<Recording>, filter: &WhitelistBlacklist<String>) {
    let nodes = start_recordings
        .into_iter()
        .map(|rec| Arc::new(MainEntity::Recording(rec)))
        .collect_vec();

    let crawler = crawler(ALISTRAL_CLIENT.musicbrainz_db.clone(), nodes);

    let crawler = crawler
        .map_ok(|entity| process_lints(entity.clone(), filter))
        .extract_future_ok()
        .buffer_unordered(8);

    pin_mut!(crawler);

    while let Some(_entity) = crawler
        .try_next()
        .await
        .expect("Couldn't get the next item")
    {}

    println!("No more data to process");
}

// === Process lints

async fn process_lints(entity: Arc<MainEntity>, filter: &WhitelistBlacklist<String>) {
    let entity = &mut entity.as_ref().clone();

    process_lint::<DashETILint>(entity, filter).await;
    process_lint::<MissingWorkLint>(entity, filter).await;
    process_lint::<MissingBarcodeLint>(entity, filter).await;
    process_lint::<MissingRemixRelLint>(entity, filter).await;
    process_lint::<SuspiciousRemixLint>(entity, filter).await;
    process_lint::<MissingRemixerRelLint>(entity, filter).await;
    process_lint::<SoundtrackWithoutDisambiguationLint>(entity, filter).await;

    println!(
        "[Processed] {}",
        entity
            .format_with_async(&MUSIBRAINZ_FMT)
            .await
            .expect("Error while formating the name of the entity")
    );
}

async fn process_lint<L: MbClippyLint>(
    entity: &mut MainEntity,
    filter: &WhitelistBlacklist<String>,
) {
    // Check if the lint is allowed
    if !filter.is_allowed(&L::get_name().to_string()) {
        return;
    }

    // Check the lint with old data

    debug!(
        "Checking Lint `{}` for `{}`",
        L::get_name(),
        entity.get_unique_id()
    );

    let Some(_lint) = L::check(&ALISTRAL_CLIENT.symphonize, entity)
        .await
        .expect("Error while processing lint")
    else {
        return;
    };

    // There might be an issue, so grab the latest data and recheck

    debug!(
        "Rechecking Lint `{}` for `{}`",
        L::get_name(),
        entity.get_unique_id()
    );

    entity
        .refetch_and_load_as_task(ALISTRAL_CLIENT.musicbrainz_db.clone())
        .await
        .expect("Couldn't refresh the entity");

    let Some(lint) = L::check(&ALISTRAL_CLIENT.symphonize, entity)
        .await
        .expect("Error while processing lint")
    else {
        return;
    };

    print_lint(&lint).await;
}

// === Printing ===

async fn print_lint<L: MbClippyLint>(lint: &L) {
    let mut report = String::new();
    writeln!(
        &mut report,
        "{}",
        format!("\n {} ", L::get_name())
            .on_truecolor_tup(lint.get_severity().get_color())
            .black()
            .bold()
    )
    .unwrap();
    writeln!(&mut report).unwrap();
    writeln!(
        &mut report,
        "{}",
        lint.get_body(&ALISTRAL_CLIENT.symphonize)
            .await
            .expect("Error while processing lint body")
    )
    .unwrap();

    // Hints
    let hints = lint
        .get_hints(&ALISTRAL_CLIENT.symphonize)
        .await
        .expect("Error while processing lint hints");
    if !hints.is_empty() {
        writeln!(&mut report).unwrap();
        for hint in hints {
            writeln!(&mut report, "{hint}").unwrap();
        }
    }

    // Links
    writeln!(&mut report).unwrap();
    writeln!(&mut report, "Links:").unwrap();
    for link in lint
        .get_links(&ALISTRAL_CLIENT.symphonize)
        .await
        .expect("Error while processing lint links")
    {
        writeln!(&mut report, "    - {link}").unwrap();
    }

    writeln!(&mut report).unwrap();
    println!("{report}\n[Enter to continue]");
    await_next();
}
