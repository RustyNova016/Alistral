use sqlx::FromRow;

pub mod upsert;
#[derive(Debug, Default, Clone, FromRow)]
pub struct LabelInfo {
    pub id: i64,
    pub catalog_number: Option<String>,
    pub label: Option<String>,
    pub release: i64,
}

impl crate::RowId for LabelInfo {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}
