use core::fmt::Display;

#[derive(Debug)]
pub struct TopScore<T: Ord + Eq> {
    pub data: T,

    pub display: String,
}

impl<T> PartialEq for TopScore<T>
where
    T: PartialEq + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl<T> Eq for TopScore<T> where T: PartialEq + Ord {}

impl<T> PartialOrd for TopScore<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl<T> Ord for TopScore<T>
where
    T: Ord + Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.data.cmp(&other.data)
    }
}

impl<T> Display for TopScore<T>
where
    T: Ord + Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}
