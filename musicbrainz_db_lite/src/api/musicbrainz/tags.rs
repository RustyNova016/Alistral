use musicbrainz_rs_nova::entity::tag::Tag as MBTag;

use crate::models::musicbrainz::tags::Tag;
use crate::models::shared_traits::has_tags::HasTags;
use crate::RowId;

impl Tag {
    pub async fn save_api_response<T: HasTags + RowId>(
        conn: &mut sqlx::SqliteConnection,
        value: MBTag,
        parent: &T,
    ) -> Result<Self, crate::Error> {
        let mut tag = Tag {
            count: value.count.map(|n| n as i64),
            name: value.name,
            score: value.score.map(|n| n as i64),
            id: Default::default(),
        };

        tag.upsert::<T>(conn, parent.get_row_id()).await?;
        Ok(tag)
    }
}
