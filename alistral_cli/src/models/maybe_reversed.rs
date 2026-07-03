#[derive(Debug, PartialEq, Eq)]
pub enum MaybeReversed<T> {
    Normal(T),
    Reversed(T),
}

impl<T> PartialOrd for MaybeReversed<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (MaybeReversed::Normal(a), Self::Normal(b)) => a.partial_cmp(b),
            (MaybeReversed::Reversed(a), Self::Reversed(b)) => b.partial_cmp(a),
            _ => None,
        }
    }
}

impl<T> Ord for MaybeReversed<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
