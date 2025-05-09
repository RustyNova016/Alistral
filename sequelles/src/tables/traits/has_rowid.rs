/// Trait for all row structs that have a row ID.
/// This is a unique incremental integer above 0.
///
/// A row id of 0 is considered as a row that isn't inserted in the database yet.
pub trait HasRowID {
    fn rowid(&self) -> i64;
}

impl HasRowID for i64 {
    fn rowid(&self) -> i64 {
        *self
    }
}

impl<T> HasRowID for &T
where
    T: HasRowID,
{
    fn rowid(&self) -> i64 {
        T::rowid(self)
    }
}

impl<T> HasRowID for Option<T>
where
    T: HasRowID,
{
    fn rowid(&self) -> i64 {
        match self {
            Some(val) => val.rowid(),
            None => 0,
        }
    }
}
