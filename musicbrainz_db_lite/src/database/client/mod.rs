use std::sync::Arc;

use listenbrainz::raw::Client as ListenbrainzClient;
use musicbrainz_rs_nova::client::MusicBrainzClient;

use crate::database::client::builder::ClientBuilder;
use crate::database::db_connection::DbConnection;

pub mod builder;

#[derive(Debug)]
pub struct DBClient {
    pub connection: DbConnection,

    pub musicbrainz_client: Arc<MusicBrainzClient>,
    pub listenbrainz_client: Arc<ListenbrainzClient>,
}

impl DBClient {
    pub fn builder() -> ClientBuilder<(), (), ()> {
        ClientBuilder::default()
    }

    #[cfg(test)]
    /// Create an in memory database with the default MB client
    pub async fn connect_in_memory_and_create() -> Result<Self, crate::Error> {
        Ok(Self::builder()
            .in_memory()
            .set_default_mb_client()
            .set_default_lb_client()
            .connect_and_migrate()
            .await?
            .build())
    }
}
