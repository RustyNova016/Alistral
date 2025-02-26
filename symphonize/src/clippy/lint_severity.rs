pub enum LintSeverity {
    MissingData,
    MissingRelation,
    WrongData,
    StyleIssue,
}

impl LintSeverity {
    pub fn get_color(&self) -> (u8, u8, u8) {
        match self {
            Self::MissingData => (32, 117, 191),
            Self::MissingRelation => (141, 102, 226),
            Self::WrongData => (191, 45, 32),
            Self::StyleIssue => (232, 133, 58),
        }
    }
}
