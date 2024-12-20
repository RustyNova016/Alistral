pub enum WhitelistBlacklist<T> {
    WhiteList(Vec<T>),
    BlackList(Vec<T>),
}

impl<T> WhitelistBlacklist<T> {
    pub fn is_allowed(&self, element: &T) -> bool
    where
        T: Eq,
    {
        match &self {
            Self::WhiteList(vals) => vals.contains(element),
            Self::BlackList(vals) => !vals.contains(element),
        }
    }
}

impl<T> Default for WhitelistBlacklist<T> {
    fn default() -> Self {
        Self::BlackList(Vec::new())
    }
}
