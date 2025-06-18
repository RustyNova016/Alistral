use genre::create_genre_table;
use release_groups::create_release_group_tables;
use sqlx::SqliteConnection;

use artists::create_artist_tables;
use label::create_label_tables;
use recordings::create_recordings_tables;
use relations::create_relation_tables;
use releases::create_release_tables;
use work::create_work_tables;

use crate::tables::musicbrainz::isrc::create_isrc_table;
use crate::tables::musicbrainz::url::create_url_tables;

pub mod artists;
pub mod genre;
pub mod gid_redirect_tables;
pub mod isrc;
pub mod label;
pub mod recordings;
pub mod relations;
pub mod release_groups;
pub mod releases;
pub mod tag;
pub mod url;
pub mod work;

pub(super) async fn generate_musicbrainz_database(
    conn: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    create_artist_tables(conn).await?;
    create_genre_table(conn).await?;
    create_recordings_tables(conn).await?;
    create_release_tables(conn).await?;
    create_release_group_tables(conn).await?;
    create_label_tables(conn).await?;
    create_url_tables(conn).await?;
    create_work_tables(conn).await?;

    create_relation_tables(conn).await?;

    create_isrc_table(conn).await?;

    Ok(())
}
