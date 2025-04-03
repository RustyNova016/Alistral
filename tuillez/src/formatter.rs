pub trait FormatWith<T> {
    type Error;

    /// Format the current element with a custom formatter.
    fn format_with(&self, ft: &T) -> Result<String, Self::Error>;
}

pub trait FormatWithAsync<T> {
    type Error;

    /// Format the current element with a custom formatter.
    fn format_with_async(
        &self,
        ft: &T,
    ) -> impl std::future::Future<Output = Result<String, Self::Error>> + Send;
}
