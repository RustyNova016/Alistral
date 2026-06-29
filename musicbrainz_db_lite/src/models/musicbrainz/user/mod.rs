use sea_query::enum_def;
use sequelles::has_rowid::HasRowID;

use crate::models::shared_traits::has_table::HasTable;

#[derive(Debug, sqlx::FromRow, Clone, PartialEq, Eq, sequelles::Table)]
#[sequelles(db_name("users"), snafu)]
#[sequelles(sqlite)]
#[sequelles(insert, insert_struct, select_unique, selsert)]
#[sequelles(primary_key(key_name = "pk", columns(id)))]
#[sequelles(unique(key_name = "name", columns(name)))]
#[enum_def(table_name = "users")]
pub struct User {
    #[sequelles(auto_increment)]
    pub id: i64,

    pub name: String,
}

impl HasRowID for User {
    fn rowid(&self) -> i64 {
        self.id
    }
}

impl HasTable for User {
    const FOREIGN_FIELD_NAME: &str = "user";
    const TABLE_NAME: &str = "users";
}
