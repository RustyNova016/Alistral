use sqlx::prelude::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct ExternalId {
    pub id: i64,
    pub recording_id: i64,
    pub ext_id: String,
    pub service: String,
    pub user_overwrite: Option<String>,
}
