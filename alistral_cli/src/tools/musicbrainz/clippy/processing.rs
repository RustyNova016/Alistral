use std::sync::Arc;

use futures::StreamExt;
use futures::stream;
use musicbrainz_db_lite::MainEntity;
use symphonize::clippy::clippy_lint::MbClippyLint;
use symphonize::clippy::lints::dash_eti::DashETILint;
use symphonize::clippy::lints::dash_eti::DashETILintRes;
use symphonize::clippy::lints::missing_release_barcode::MissingBarcodeLint;
use symphonize::clippy::lints::missing_remix_rel::MissingRemixRelLint;
use symphonize::clippy::lints::missing_remixer_rel::MissingRemixerRelLint;
use symphonize::clippy::lints::missing_work::MissingWorkLint;
use symphonize::clippy::lints::soundtrack_without_disambiguation::SoundtrackWithoutDisambiguationLint;
use symphonize::clippy::lints::suspicious_remix::SuspiciousRemixLint;
use tokio::sync::Semaphore;
use tracing::debug;
use tracing::info;
use tuillez::formatter::FormatWithAsync;

use crate::ALISTRAL_CLIENT;
use crate::tools::musicbrainz::clippy::display::LintActions;
use crate::tools::musicbrainz::clippy::display::print_lint;
use crate::utils::constants::MUSIBRAINZ_FMT;
use crate::utils::whitelist_blacklist::WhitelistBlacklist;

// === Main Processing ===

pub(super) async fn process_lints(
    entity: Arc<MainEntity>,
    filter: &WhitelistBlacklist<String>,
) -> bool {
    let entity = &mut entity.as_ref().clone();

    match process_lint::<DashETILint>(entity, filter).await {
        LintProcessAction::Exit => return false,
        LintProcessAction::Continue => {}
    }

    match process_lint::<MissingBarcodeLint>(entity, filter).await {
        LintProcessAction::Exit => return false,
        LintProcessAction::Continue => {}
    }

    match process_lint::<MissingRemixRelLint>(entity, filter).await {
        LintProcessAction::Exit => return false,
        LintProcessAction::Continue => {}
    }

    match process_lint::<MissingRemixerRelLint>(entity, filter).await {
        LintProcessAction::Exit => return false,
        LintProcessAction::Continue => {}
    }

    match process_lint::<SuspiciousRemixLint>(entity, filter).await {
        LintProcessAction::Exit => return false,
        LintProcessAction::Continue => {}
    }

    match process_lint::<SoundtrackWithoutDisambiguationLint>(entity, filter).await {
        LintProcessAction::Exit => return false,
        LintProcessAction::Continue => {}
    }

    match process_lint::<MissingWorkLint>(entity, filter).await {
        LintProcessAction::Exit => return false,
        LintProcessAction::Continue => {}
    }

    if PROCESS_LOCK.try_acquire().is_ok() {
        info!(
            "Checked {}",
            entity
                .format_with_async(&MUSIBRAINZ_FMT)
                .await
                .expect("Error while formating the name of the entity")
        );
    }

    true
}

enum LintProcessAction {
    Exit,
    Continue,
    //Reload
}

static PROCESS_LOCK: Semaphore = Semaphore::const_new(1);

async fn process_lint<L: MbClippyLint>(
    entity: &mut MainEntity,
    filter: &WhitelistBlacklist<String>,
) -> LintProcessAction
where
    <L as symphonize::clippy::clippy_lint::MbClippyLint>::Result:
        std::marker::Sync + 'static + Send,
{
    // Check if the lint is allowed
    if !filter.is_allowed(&L::get_name().to_string()) {
        return LintProcessAction::Continue;
    }

    debug!(
        "Lazy checking Lint `{}` for `{}`",
        L::get_name(),
        entity.get_unique_id()
    );

    L::prefetch_entities(&ALISTRAL_CLIENT.symphonize, entity)
        .await
        .expect("Couldn't get data for the lint");

    if L::check(&ALISTRAL_CLIENT.symphonize, entity)
        .await
        .expect("Error while processing lint")
        .is_empty()
    {
        return LintProcessAction::Continue;
    }

    debug!(
        "Rechecking Lint `{}` for `{}`",
        L::get_name(),
        entity.get_unique_id()
    );

    entity
        .refetch_and_load_as_task(ALISTRAL_CLIENT.musicbrainz_db.clone())
        .await
        .expect("Couldn't refresh the entity");

    let lints = L::check(&ALISTRAL_CLIENT.symphonize, entity)
        .await
        .expect("Error while processing lint");

    // We only allow one process to get up to there to avoid multiple lint printed
    let _permit = PROCESS_LOCK.acquire().await.unwrap();

    println!("Found {} lint of {}", lints.len(), L::get_name());
    for lint in lints {
        let user_action = tokio::spawn(async move {
            let lint = lint;
            print_lint(&lint).await
        })
        .await
        .unwrap();

        match user_action {
            LintActions::Exit => return LintProcessAction::Exit,
            //LintActions::Done => return LintProcessAction::Reload,
            LintActions::Done => {}
        }
    }

    info!("Checked {}", L::get_name());

    LintProcessAction::Continue
}
