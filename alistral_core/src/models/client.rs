use listenbrainz::raw::Client as ListenbrainzClient;
use musicbrainz_db_lite::client::MusicBrainzClient;
use musicbrainz_db_lite::database::client::DBClient;

pub struct AlistralClient {
    pub musicbrainz_rs: MusicBrainzClient,
    pub listenbrainz: ListenbrainzClient,

    pub musicbrainz_db: DBClient,
}