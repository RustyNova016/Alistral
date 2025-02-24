pub trait FindBy<T>: Sized {
    fn find_by(
        conn: &mut sqlx::SqliteConnection,
        id: T,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send;
}
