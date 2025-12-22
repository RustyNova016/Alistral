use std::sync::Arc;

use interzic::InterzicClient;
use interzic::models::services::subsonic::SubsonicClient;
use musicbrainz_db_lite::DBClient;
use musicbrainz_db_lite::MusicBrainzClient;

use crate::models::client::AlistralCliClient;
use crate::models::config::global_config::CONFIG;
use crate::utils::constants::INTERZIC_DB;

impl AlistralCliClient {
    pub(super) async fn create_interzic(
        musicbrainz_rs: Arc<MusicBrainzClient>,
        musicbrainz_db: Arc<DBClient>,
    ) -> Arc<InterzicClient> {
        let mut builder = InterzicClient::new_builder();
        builder
            .create_database_if_missing(&INTERZIC_DB)
            .expect("Couldn't create interzic database");

        builder
            .read_database(INTERZIC_DB.clone().to_str().unwrap())
            .expect("Couldn't connect to interzic database");

        builder
            .migrate_database()
            .await
            .expect("Couldn't migrate interzic database");

        let mut client = builder.build().expect("Couldn't initialize Interzic");

        client.set_musicbrainz_client(musicbrainz_rs);
        client.set_listenbrainz_client(Arc::new(listenbrainz::raw::Client::default()));
        client.set_musicbrainz_db_lite_client(musicbrainz_db);

        #[cfg(feature = "youtube")]
        set_youtube(&mut client).await;

        #[cfg(feature = "subsonic")]
        set_subsonic(&mut client).await;

        Arc::new(client)
    }
}

#[cfg(feature = "youtube")]
async fn set_youtube(client: &mut InterzicClient) {
    use crate::utils::constants::YT_SECRET_FILE;

    if YT_SECRET_FILE.exists() {
        use crate::utils::constants::TOKENCACHE;

        client
            .set_youtube_client(&YT_SECRET_FILE, &TOKENCACHE)
            .await
            .expect("Couldn't create the youtube client");
    }
}

async fn set_subsonic(client: &mut InterzicClient) {
    let config = CONFIG.config.read().await;

    for conf in &config.interzic.subsonic_clients {
        client.add_subsonic_client(SubsonicClient::new(
            conf.name.to_string(),
            &conf.url,
            &conf.username,
            &conf.password,
        ));
    }
}
