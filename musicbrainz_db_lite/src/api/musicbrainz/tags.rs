use musicbrainz_rs::entity::tag::Tag as MBTag;
use sequelles::has_rowid::HasRowID;

use crate::models::musicbrainz::tags::Tag;
use crate::models::shared_traits::has_tags::HasTags;

impl Tag {
    pub async fn save_api_response<T: HasTags + HasRowID>(
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

        tag.upsert::<T>(conn, parent.rowid()).await?;
        Ok(tag)
    }
}
