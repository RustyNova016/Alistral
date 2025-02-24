use std::fs;
use std::fs::File;
use std::io::Read as _;
use std::path::PathBuf;
use std::process::Command;

pub fn load_schema_sql(path: PathBuf) -> String {
    let mut file = File::open(path).unwrap();

    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

pub async fn load_database_schema(conn: &mut sqlx::SqliteConnection, path: PathBuf) {
    let schema = load_schema_sql(path);
    sqlx::query(&schema).execute(conn).await.unwrap();
}

pub fn get_database_schema(db_path: &str) -> String {
    let out = Command::new("sqlite3")
        .arg(db_path)
        .arg(".dump ")
        .output()
        .unwrap();

    String::from_utf8(out.stdout).unwrap()
}

pub fn setup_database_file(path: &str) {
    if std::fs::exists(path).unwrap() {
        fs::remove_file(path).unwrap();
    }

    File::create_new(path).unwrap();
}

pub fn get_schema_diff(dba_path: &str, dbb_path: &str) -> String {
    let out = Command::new("sqldiff")
        .arg(dba_path)
        .arg(dbb_path)
        .output()
        .unwrap();

    String::from_utf8(out.stdout).unwrap()
}
