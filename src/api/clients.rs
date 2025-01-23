use alistral_core::models::client::AlistralClient;
use listenbrainz::raw::Client as ListenbrainzClient;

use crate::database::connect_and_setup;
use crate::database::get_db_client;
use crate::models::config::Config;

pub async fn create_client() -> AlistralClient {
    let config = Config::load_or_panic();
    let config = config.read_or_panic();

    let listenbrainz = ListenbrainzClient::new_with_url(&config.listenbrainz_url);
    drop(config);
    AlistralClient {
        musicbrainz_rs: Default::default(),
        listenbrainz,
        musicbrainz_db: connect_and_setup().await.unwrap(),
    }
}
