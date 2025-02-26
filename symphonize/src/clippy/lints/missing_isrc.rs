use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;


pub struct MissingIsrcLint {}

impl MissingIsrcLint {
    pub fn check(conn: &mut sqlx::SqliteConnection, entity: MainEntity) -> Result<(), Self> {
        let MainEntity::Recording(recording) = entity else {return Ok(())};

        recording.

        Ok(())
    }
}