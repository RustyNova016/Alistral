use core::fmt::Write as _;
use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::LazyLock;

use alistral_core::cli::colors::AlistralColors as _;
use clap::Parser;
use futures::SinkExt;
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;
use futures::join;
use futures::pin_mut;
use itertools::Itertools;
use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::main_entities::crawler::crawler;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use streamies::TryStreamies;
use symphonize::clippy::clippy_lint::MbClippyLint;
use tokio::sync::Semaphore;
use tuillez::OwoColorize as _;

use crate::ALISTRAL_CLIENT;
use crate::database::interfaces::statistics_data::recording_stats;
use crate::models::config::Config;
use crate::tools::musicbrainz::clippy::mb_clippy::mb_clippy_poller;
use crate::tools::musicbrainz::clippy::mb_clippy::mb_clippy_stream;
use crate::tools::musicbrainz::clippy::sambl_check::samble_clippy_poller;
use crate::tools::musicbrainz::clippy::sambl_check::samble_clippy_stream;
use crate::utils::cli::await_next;
use crate::utils::cli::read_mbid_from_input;
use crate::utils::whitelist_blacklist::WhitelistBlacklist;

pub mod mb_clippy;
pub mod sambl_check;

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

    /// Sort the initial recordings by their name
    #[arg(short, long)]
    pub sort: bool
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
        let mut start = match &self.start_mbid {
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
        };

        if self.sort {
            start.sort_unstable_by_key(|rec| rec.title.to_owned());
        }

        start
    }
}

pub async fn mb_clippy(
    start_recordings: Vec<Recording>,
    filter: Arc<WhitelistBlacklist<String>>,
) {
    PROCESSED_COUNT.store(1, Ordering::Release);
    let nodes = start_recordings
        .into_iter()
        .map(|rec| Arc::new(MainEntity::Recording(rec)))
        .collect_vec();

    // Prepare clippys:
    let (mb_send, mb_stream) = mb_clippy_stream(filter.clone());
    let (sambl_send, sambl_stream) = samble_clippy_stream(&filter);

    let crawler = crawler(ALISTRAL_CLIENT.musicbrainz_db.clone(), nodes)
        .map_ok(|entity| {
            let mut mb_send = mb_send.clone();
            let mut sambl_send = sambl_send.clone();
            async move {
                mb_send.send(entity.clone()).await.unwrap();
                sambl_send.send(entity.clone()).await.unwrap();
            }
        })
        .extract_future_ok()
        .buffered(16)
        .map_err(crate::Error::from);

    let _ = join!(
        crawler_poller(crawler),
        mb_clippy_poller(mb_stream),
        samble_clippy_poller(sambl_stream)
    );

    println!("No more data to process");
}

pub async fn crawler_poller(
    stream: impl Stream<Item = Result<(), crate::Error>>,
) -> Result<(), crate::Error> {
    pin_mut!(stream);
    while let Some(_val) = stream.try_next().await? {}
    Ok(())
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

// async fn process_multi_lint(entity: &mut MainEntity) {
//     let mut lints = MissingSamblReleaseLint::get_all_lints(&ALISTRAL_CLIENT.symphonize, entity)
//         .await
//         .unwrap();

//     while let Some((album, lint)) = lints.try_next().await.unwrap() {
//         if let Some(lint) = lint {
//             print_lint(&lint).await;
//         }

//         let _lock = PRINT_LOCK
//             .acquire()
//             .await
//             .expect("Print lock has been closed");

//         info!(
//             "[Processed - {}] [SAMBL] `{}`",
//             PROCESSED_COUNT.fetch_add(1, Ordering::AcqRel),
//             album.spotify_name
//         );
//     }
// }
