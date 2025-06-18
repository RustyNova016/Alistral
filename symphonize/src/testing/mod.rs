use musicbrainz_db_lite::MainEntity;
use musicbrainz_db_lite::models::shared_traits::find_by_mbid::FindByMBID;

use crate::SymphonyzeClient;
use crate::clippy::clippy_lint::MbClippyLint;

async fn trigger_lint<Lint, Entity>(client: &SymphonyzeClient, mbid: &str) -> bool
where
    Lint: MbClippyLint,
    Entity: FindByMBID + Into<MainEntity>,
{
    let data = Entity::find_by_mbid_with_pool(&client.mb_database, mbid)
        .await
        .unwrap()
        .expect("Couldn't find provided entity MBID");

    Lint::check(client, &data.into()).await.unwrap().is_some()
}

pub async fn should_trigger_lint<Lint, Entity>(client: &SymphonyzeClient, mbid: &str)
where
    Lint: MbClippyLint,
    Entity: FindByMBID + Into<MainEntity>,
{
    if !trigger_lint::<Lint, Entity>(client, mbid).await {
        panic!(
            "MBID `{}` should have triggered lint `{}`, but didn't",
            mbid,
            Lint::get_name()
        );
    }
}

pub async fn shouldnt_trigger_lint<Lint, Entity>(client: &SymphonyzeClient, mbid: &str)
where
    Lint: MbClippyLint,
    Entity: FindByMBID + Into<MainEntity>,
{
    if trigger_lint::<Lint, Entity>(client, mbid).await {
        panic!(
            "MBID `{}` have triggered lint `{}`, while it shouldn't",
            mbid,
            Lint::get_name()
        );
    }
}

pub fn test_name() -> String {
    std::thread::current().name().unwrap().to_string()
}
