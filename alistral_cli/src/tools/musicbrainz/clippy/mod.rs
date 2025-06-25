use core::fmt::Write as _;
use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::LazyLock;

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
use symphonize::clippy::lints::label_as_artist::LabelAsArtistLint;
use symphonize::clippy::lints::missing_artist_link::MissingArtistLink;
use symphonize::clippy::lints::missing_isrc::MissingISRCLint;
use symphonize::clippy::lints::missing_recording_link::MissingRecordingLink;
use symphonize::clippy::lints::missing_release_barcode::MissingBarcodeLint;
use symphonize::clippy::lints::missing_remix_rel::MissingRemixRelLint;
use symphonize::clippy::lints::missing_remixer_rel::MissingRemixerRelLint;
use symphonize::clippy::lints::missing_work::MissingWorkLint;
use symphonize::clippy::lints::soundtrack_without_disambiguation::SoundtrackWithoutDisambiguationLint;
use symphonize::clippy::lints::suspicious_remix::SuspiciousRemixLint;
use tokio::sync::Semaphore;
use tracing::debug;
use tracing::info;
use tuillez::OwoColorize as _;
use tuillez::formatter::FormatWithAsync;

use crate::ALISTRAL_CLIENT;
use crate::database::interfaces::statistics_data::recording_stats;
use crate::models::config::Config;
use crate::utils::cli::await_next;
use crate::utils::cli::read_mbid_from_input;
use crate::utils::constants::MUSIBRAINZ_FMT;
use crate::utils::whitelist_blacklist::WhitelistBlacklist;

static REFETCH_LOCK: LazyLock<Arc<Semaphore>> = LazyLock::new(|| Arc::new(Semaphore::new(1)));
static PRINT_LOCK: LazyLock<Arc<Semaphore>> = LazyLock::new(|| Arc::new(Semaphore::new(1)));
static PROCESSED_COUNT: AtomicU64 = AtomicU64::new(0);

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

        mb_clippy(self.get_start_recordings().await, Arc::new(filter)).await;
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

pub async fn mb_clippy(start_recordings: Vec<Recording>, filter: Arc<WhitelistBlacklist<String>>) {
    PROCESSED_COUNT.store(1, Ordering::Release);
    let nodes = start_recordings
        .into_iter()
        .map(|rec| Arc::new(MainEntity::Recording(rec)))
        .collect_vec();

    let crawler = crawler(ALISTRAL_CLIENT.musicbrainz_db.clone(), nodes);

    let crawler = crawler
        .map_ok(|entity| {
            let filter = filter.clone();
            tokio::spawn(async move { process_lints(entity.clone(), filter.clone()).await })
        })
        .extract_future_ok()
        .buffer_unordered(16);

    pin_mut!(crawler);

    while let Some(_entity) = crawler
        .try_next()
        .await
        .expect("Couldn't get the next item")
        .transpose()
        .expect("Join error")
    {}

    println!("No more data to process");
}

// === Process lints

async fn process_lints(entity: Arc<MainEntity>, filter: Arc<WhitelistBlacklist<String>>) {
    let entity = &mut entity.as_ref().clone();

    process_lint::<DashETILint>(entity, &filter).await;
    process_lint::<MissingISRCLint>(entity, &filter).await;
    process_lint::<MissingWorkLint>(entity, &filter).await;
    process_lint::<LabelAsArtistLint>(entity, &filter).await;
    process_lint::<MissingArtistLink>(entity, &filter).await;
    process_lint::<MissingBarcodeLint>(entity, &filter).await;
    process_lint::<MissingRemixRelLint>(entity, &filter).await;
    process_lint::<SuspiciousRemixLint>(entity, &filter).await;
    process_lint::<MissingRecordingLink>(entity, &filter).await;
    process_lint::<MissingRemixerRelLint>(entity, &filter).await;
    process_lint::<SoundtrackWithoutDisambiguationLint>(entity, &filter).await;

    let _lock = PRINT_LOCK
        .acquire()
        .await
        .expect("Print lock has been closed");

    info!(
        "[Processed - {}] {}",
        PROCESSED_COUNT.fetch_add(1, Ordering::AcqRel),
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
    // Also prevent others from fetching data that might get stale after the user fix this lint

    let _lock = REFETCH_LOCK
        .acquire()
        .await
        .expect("Refetch lock has been closed");

    debug!(
        "Rechecking Lint `{}` for `{}`",
        L::get_name(),
        entity.get_unique_id()
    );

    L::refresh_data(&ALISTRAL_CLIENT.symphonize, entity)
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
    let _lock = PRINT_LOCK
        .acquire()
        .await
        .expect("Print lock has been closed");

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
