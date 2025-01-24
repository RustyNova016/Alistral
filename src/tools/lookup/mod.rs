pub mod recording;
use crate::models::cli::lookup::LookupTarget;
use recording::lookup_recording;

pub async fn lookup_command(
    conn: &mut sqlx::SqliteConnection,
    username: &str,
    id: &str,
    target: LookupTarget,
) -> color_eyre::Result<()> {
    match target {
        LookupTarget::Recording => lookup_recording(conn, username, id).await,
    }
}
