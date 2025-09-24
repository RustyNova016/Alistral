use musicbrainz_rs::entity::label::LabelInfo as MBLabelInfo;

use crate::Label;
use crate::LabelInfo;

impl LabelInfo {
    pub async fn save_api_response(
        conn: &mut sqlx::SqliteConnection,
        value: Vec<MBLabelInfo>,
        release_id: i64,
    ) -> Result<Vec<Self>, crate::Error> {
        let mut converteds = Vec::new();
        for item in value {
            let new_media = Self {
                id: Default::default(),
                release: release_id,
                catalog_number: item.catalog_number,
                label: item.label.clone().map(|l| l.id),
            };

            if let Some(label) = item.label {
                Label::save_api_response(&mut *conn, label).await?;
            }

            let new_media = new_media.upsert(&mut *conn).await?;

            converteds.push(new_media);
        }

        Ok(converteds)
    }
}
