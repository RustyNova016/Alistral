use musicbrainz_rs_nova::entity::genre::Genre as MBGenre;

use crate::RowId;
use crate::models::musicbrainz::genre::Genre;
use crate::models::musicbrainz::genre::genre_tag::GenreTag;
use crate::models::shared_traits::has_genre::HasGenres;

impl Genre {
    pub async fn save_api_response(
        conn: &mut sqlx::SqliteConnection,
        value: MBGenre,
    ) -> Result<Self, crate::Error> {
        let mut genre = Genre {
            mbid: value
                .id
                .expect("The mbid of the genre should always be present"),
            disambiguation: value.disambiguation,
            name: value.name,
            id: Default::default(),
        };

        genre.upsert(conn).await?;

        Ok(genre)
    }
}

impl GenreTag {
    pub async fn save_api_response<T: HasGenres + RowId>(
        conn: &mut sqlx::SqliteConnection,
        value: MBGenre,
        parent: &T,
    ) -> Result<Self, crate::Error> {
        let genre = Genre::save_api_response(conn, value.clone()).await?;

        let mut genre_tag = GenreTag {
            count: value.count.map(|n| n as i64),
            genre: genre.id,
            id: Default::default(),
        };

        genre_tag.upsert::<T>(conn, parent.get_row_id()).await?;
        Ok(genre_tag)
    }
}
