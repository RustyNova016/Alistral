use sqlx::SqliteConnection;

pub mod listenbrainz;
pub mod musicbrainz;

/// This trait is implemented by all the entities that are able to be saved to the database
pub trait SaveToDatabase {
    type ReturnedData;

    /// Save the object into the database, with a Client or transaction (without commit)
    fn save(
        self,
        conn: &mut SqliteConnection,
    ) -> impl std::future::Future<Output = Result<Self::ReturnedData, crate::Error>> + Send;
}
