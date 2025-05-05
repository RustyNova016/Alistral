use std::sync::Arc;

use musicbrainz_db_lite::MainEntity;
use symphonize::clippy::clippy_lint::MbClippyLint;
use symphonize::clippy::lints::dash_eti::DashETILint;
use tracing::debug;
use tuillez::formatter::FormatWithAsync;

use crate::utils::constants::MUSIBRAINZ_FMT;
use crate::utils::whitelist_blacklist::WhitelistBlacklist;
use crate::ALISTRAL_CLIENT;

pub(super) async fn process_lints(entity: Arc<MainEntity>, filter: &WhitelistBlacklist<String>) {
    let entity = &mut entity.as_ref().clone();

    process_lint::<DashETILint>(entity, filter).await;

    println!(
        "[Processed] {}",
        entity
            .format_with_async(&MUSIBRAINZ_FMT)
            .await
            .expect("Error while formating the name of the entity")
    );
}

enum LintProcessAction {
    PrefetchAndCheck
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

    L::prefetch_entities(&ALISTRAL_CLIENT.symphonize, entity)
        .await
        .expect("Couldn't get data for the lint");

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

