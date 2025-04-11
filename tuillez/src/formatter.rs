/// Trait to format an element with a custom formatter.
///
/// See [`FormatWithAsync`] for an async version of this trait.
pub trait FormatWith<T> {
    type Error;

    /// Format the current element with a custom formatter.
    fn format_with(&self, ft: &T) -> Result<String, Self::Error>;
}

/// A trait for formatting an element with a custom formatter asynchronously.
///
/// This trait is similar to [`FormatWith`], but it allows for asynchronous formatting.
pub trait FormatWithAsync<T> {
    type Error;

    /// Format the current element with a custom formatter, asynchronously.
    fn format_with_async(
        &self,
        ft: &T,
    ) -> impl std::future::Future<Output = Result<String, Self::Error>> + Send;
}
