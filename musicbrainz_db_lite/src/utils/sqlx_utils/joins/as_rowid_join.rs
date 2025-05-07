pub trait AsRowIdJoin {
    type Output;

    fn as_rowid_join(&self) -> Self::Output;
}
