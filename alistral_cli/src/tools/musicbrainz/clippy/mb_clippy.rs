use core::sync::atomic::Ordering;
use std::sync::Arc;

use futures::Stream;
use futures::StreamExt as _;
use futures::TryStreamExt;
use futures::channel::mpsc::Sender;
use futures::channel::mpsc::channel;
use futures::pin_mut;
use musicbrainz_db_lite::MainEntity;
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
use tokio::task::JoinError;
use tracing::debug;
use tuillez::formatter::FormatWithAsync as _;

use crate::ALISTRAL_CLIENT;
use crate::tools::musicbrainz::clippy::PRINT_LOCK;
use crate::tools::musicbrainz::clippy::PROCESSED_COUNT;
use crate::tools::musicbrainz::clippy::REFETCH_LOCK;
use crate::tools::musicbrainz::clippy::print_lint;
use crate::utils::constants::MUSIBRAINZ_FMT;
use crate::utils::whitelist_blacklist::WhitelistBlacklist;

pub async fn mb_clippy_poller(
    stream: impl Stream<Item = Result<(), JoinError>>,
) -> Result<(), JoinError> {
    pin_mut!(stream);
    while let Some(_val) = stream.try_next().await? {}

    Ok(())
}

pub fn mb_clippy_stream(
    filter: Arc<WhitelistBlacklist<String>>,
) -> (
    Sender<Arc<MainEntity>>,
    impl Stream<Item = Result<(), JoinError>>,
) {
    let (entity_send, entity_stream) = channel::<Arc<MainEntity>>(10);

    let stream = entity_stream
        .map(move |entity| {
            let filter = filter.clone();
            tokio::spawn(async move {
                process_lints(entity.clone(), filter.clone()).await;
            })
        })
        .buffer_unordered(16);

    (entity_send, stream)
}

// === Process lints

async fn process_lints(entity: Arc<MainEntity>, filter: Arc<WhitelistBlacklist<String>>) {
    let entity = &mut entity.as_ref().to_owned();

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

    println!(
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

    if L::check(&ALISTRAL_CLIENT.symphonize, entity)
        .await
        .expect("Error while processing lint")
        .is_none()
    {
        return;
    }

    // There might be an issue, so grab the latest data and recheck
    // Also prevent others from fetching data that might get stale after the user fix this lint

    let _lock = REFETCH_LOCK
        .acquire()
        .await
        .expect("Refetch lock has been closed");

    let Some(lint) = recheck_lint::<L>(entity).await else {
        return;
    };

    print_lint(&lint).await;
}

//#[instrument(fields(indicatif.pb_show = tracing::field::Empty))]
pub async fn recheck_lint<L: MbClippyLint>(entity: &mut MainEntity) -> Option<L> {
    //pg_spinner!("Verifying lint...");

    debug!(
        "Rechecking Lint `{}` for `{}`",
        L::get_name(),
        entity.get_unique_id()
    );

    L::refresh_data(&ALISTRAL_CLIENT.symphonize, entity)
        .await
        .expect("Couldn't refresh the entity");

    L::check(&ALISTRAL_CLIENT.symphonize, entity)
        .await
        .expect("Error while processing lint")
}
