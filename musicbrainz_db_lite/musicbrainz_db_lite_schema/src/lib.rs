use sqlx::migrate::MigrateError;
use sqlx::Connection;
use tables::create_listenbrainz_tables;
use tables::create_musicbrainz_tables;
use tables::listenbrainz::generate_listenbrainz_database;

pub mod tables;
#[cfg(test)]
pub mod testing;

pub async fn create_and_migrate(conn: &mut sqlx::SqliteConnection) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(conn).await
}

/// Create the latest version of the database, and isn't meant to be used for production.
///
/// See `create_and_migrate` to create the database / update it when needed
#[allow(dead_code)] // It cannot see that it is used in the test below
async fn create_latest_database(conn: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
    let mut trans: sqlx::Transaction<'_, sqlx::Sqlite> = conn.begin().await?;

    create_musicbrainz_tables(&mut trans).await?;
    create_listenbrainz_tables(&mut trans).await?;
    generate_listenbrainz_database(&mut trans).await?;
    trans.commit().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    use crate::create_and_migrate;
    use crate::create_latest_database;
    use crate::testing::get_database_schema;
    use crate::testing::get_schema_diff;
    use crate::testing::setup_database_file;

    async fn should_generate_schema() {
        // Set up db file
        setup_database_file("./schema.db");
        let sql_pool = sqlx::SqlitePool::connect_lazy("sqlite:./schema.db").unwrap();

        // Create Database
        create_latest_database(&mut sql_pool.acquire().await.unwrap())
            .await
            .unwrap();

        //assert!(check_db_integrity(&client).await.is_ok_and(|v| v));

        let out = Command::new("sqlite3")
            .arg("./schema.db")
            .arg(".dump ")
            .output()
            .unwrap();

        File::create("./schema.sql")
            .unwrap()
            .write_all(&out.stdout)
            .unwrap();
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn should_migrate_schema() {
        // Test generation first
        should_generate_schema().await;

        // Set up db file
        setup_database_file("./migration_test.db");
        let db = sqlx::SqlitePool::connect_lazy("sqlite:./migration_test.db").unwrap();
        let mut conn = db.acquire().await.unwrap();

        create_and_migrate(&mut conn).await.unwrap();

        // Database has been migrated. Let's check that it's up to par with the main one
        // ... But first, we need to drop _sqlx_migrations. While migrating this table is automatically created,
        // but we don't want it in our public schema
        sqlx::query("DROP TABLE _sqlx_migrations")
            .execute(&mut *conn)
            .await
            .unwrap();

        let migrated_schema = get_database_schema("./migration_test.db");

        let diffs = get_schema_diff("./migration_test.db", "./schema.db");

        if !diffs.is_empty() {
            let mut file = File::create("./migration_test_schema.sql").unwrap();

            write!(file, "{}", migrated_schema).unwrap();

            panic!("\nThe migration schema hasn't been updated properly! SQLDiff output (Missing in migration): \n\n{}", diffs)
        }
    }
}
