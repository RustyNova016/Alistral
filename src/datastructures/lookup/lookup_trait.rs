pub trait LookupTrait<T> {
    async fn to_string(&self) -> Result<String, crate::Error>;
}