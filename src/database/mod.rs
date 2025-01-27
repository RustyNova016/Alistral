pub mod musicbrainz;
use std::fs::{self};
use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::OnceLock;

use directories::BaseDirs;
use musicbrainz_db_lite::database::client::DBClient;
use sqlx::Pool;
use sqlx::Sqlite;

use crate::tools::cache::delete_database;
use crate::utils::println_cli;

pub mod cleanup;
pub mod listenbrainz;

static MUSICBRAINZ_LITE: OnceLock<DBClient> = OnceLock::new();

pub static RELEASE_DB_LOCATION: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = BaseDirs::new()
        .expect("Couldn't find the standard cache directory. Is your system an oddball one?")
        .cache_dir()
        .to_path_buf();

    path.push("alistral");

    if !fs::exists(&path).unwrap() {
        fs::create_dir_all(&path).expect("Couldn't create cache directory");
    }
    path.push("data.db");

    path
});

pub static DEBUG_DB_LOCATION: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = BaseDirs::new()
        .expect("Couldn't find the standard cache directory. Is your system an oddball one?")
        .cache_dir()
        .to_path_buf();

    path.push("alistral");
    path.push("debug");

    if !fs::exists(&path).unwrap() {
        fs::create_dir_all(&path).expect("Couldn't create cache directory");
    }
    path.push("debug_db.db");

    path
});

pub static DB_LOCATION: LazyLock<PathBuf> = LazyLock::new(|| {
    #[cfg(debug_assertions)]
    return DEBUG_DB_LOCATION.clone();

    #[cfg(not(debug_assertions))]
    return RELEASE_DB_LOCATION.clone();
});

pub async fn get_db_client() -> &'static DBClient {
    if let Some(conn) = MUSICBRAINZ_LITE.get() {
        return conn;
    }

    let client = connect_and_setup().await.unwrap();

    MUSICBRAINZ_LITE.get_or_init(|| client)
}

pub async fn get_conn() -> sqlx::pool::PoolConnection<sqlx::Sqlite> {
    get_db_client()
        .await
        .connection
        .acquire()
        .await
        .expect("Couldn't get connection to the SQLite database")
}

/// Try to connect to the database if the file is present
async fn try_connect_to_db() -> Result<DBClient, crate::Error> {
    if std::fs::exists(DB_LOCATION.to_str().unwrap()).unwrap() {
        return Ok(DBClient::connect(DB_LOCATION.to_str().unwrap()).await?);
    }

    Err(crate::Error::MissingDatabaseFile(
        DB_LOCATION.to_string_lossy().to_string(),
    ))
}

async fn connect_and_setup() -> Result<DBClient, crate::Error> {
    match try_connect_to_db().await {
        Ok(db) => Ok(db),
        Err(crate::Error::MissingDatabaseFile(_)) => {
            println_cli("Creating database file");
            Ok(DBClient::create_database_file(&DB_LOCATION.to_string_lossy()).await?)
        }
        Err(crate::Error::MusicbrainzDBLiteError(err)) => {
            if has_migration_log().await? {
                return Err(err.into());
            }

            delete_database(&DB_LOCATION)?;
            Ok(DBClient::create_database_file(&DB_LOCATION.to_string_lossy()).await?)
        }
        Err(err) => Err(err),
    }
}

async fn has_migration_log() -> Result<bool, crate::Error> {
    let pool = Pool::<Sqlite>::connect_lazy(&format!("sqlite:{}", DB_LOCATION.to_string_lossy()))?;

    let result = sqlx::query("SELECT * FROM `_sqlx_migrations` LIMIT 1")
        .fetch_optional(&pool)
        .await?;

    Ok(result.is_some())
}
