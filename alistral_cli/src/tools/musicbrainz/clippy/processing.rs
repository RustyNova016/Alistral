use std::sync::Arc;

use futures::StreamExt;
use futures::stream;
use musicbrainz_db_lite::MainEntity;
use symphonize::clippy::clippy_lint::MbClippyLint;
use symphonize::clippy::lints::dash_eti::DashETILint;
use tracing::debug;
use tuillez::formatter::FormatWithAsync;

use crate::tools::musicbrainz::clippy::display::print_lint;
use crate::tools::musicbrainz::clippy::display::LintActions;
use crate::ALISTRAL_CLIENT;
use crate::utils::constants::MUSIBRAINZ_FMT;
use crate::utils::whitelist_blacklist::WhitelistBlacklist;

// === Lazy Processing ===

/// Look if any lints get triggered by the already cached data
pub(super) async fn process_lints_lazy(
    entity: Arc<MainEntity>,
    filter: &WhitelistBlacklist<String>,
) -> bool {
    let entity = &mut entity.as_ref().clone();

    stream::iter(vec![process_lint_lazy::<DashETILint>(entity, filter)])
        .any(|lint| lint)
        .await
}

/// Lazyly check if an entity trigger a lint. Return true if the lint has been trigered
async fn process_lint_lazy<L: MbClippyLint>(
    entity: &mut MainEntity,
    filter: &WhitelistBlacklist<String>,
) -> bool {
    // Check if the lint is allowed
    if !filter.is_allowed(&L::get_name().to_string()) {
        return false;
    }

    debug!(
        "Lazy checking Lint `{}` for `{}`",
        L::get_name(),
        entity.get_unique_id()
    );

    L::prefetch_entities(&ALISTRAL_CLIENT.symphonize, entity)
        .await
        .expect("Couldn't get data for the lint");

    !L::check(&ALISTRAL_CLIENT.symphonize, entity)
        .await
        .expect("Error while processing lint")
        .is_empty()
}

// === Main Processing ===

pub(super) async fn process_lints(entity: Arc<MainEntity>, filter: &WhitelistBlacklist<String>) {
    let entity = &mut entity.as_ref().clone();

    process_lint::<DashETILint>(entity, filter).await;
}

enum LintProcessAction {
    Exit,
    Continue,
    //Reload
}

async fn process_lint<L: MbClippyLint>(
    entity: &mut MainEntity,
    filter: &WhitelistBlacklist<String>,
) -> LintProcessAction {
    // Check if the lint is allowed
    if !filter.is_allowed(&L::get_name().to_string()) {
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

    for lint in lints {
        match print_lint(&lint).await {
            LintActions::Exit => return LintProcessAction::Exit,
            //LintActions::Done => return LintProcessAction::Reload,
            LintActions::Done => {}
        }
    }

    return LintProcessAction::Continue;
}
