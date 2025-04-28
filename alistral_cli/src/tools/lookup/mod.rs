pub mod recording;
use crate::models::cli::lookup::LookupTarget;
use recording::lookup_recording;
use tuillez::fatal_error::FatalError;

pub async fn lookup_command(
    conn: &mut sqlx::SqliteConnection,
    username: &str,
    id: &str,
    target: LookupTarget,
) -> Result<(), FatalError> {
    match target {
        LookupTarget::Recording => lookup_recording(conn, username, id).await,
    }
}
