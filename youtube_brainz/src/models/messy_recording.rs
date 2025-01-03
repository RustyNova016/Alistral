use core::fmt::Display;

pub struct MessyRecording {
    pub title: String,
    pub artist_credits: String,
    pub release: String,
}

impl Display for MessyRecording {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.artist_credits, self.title)?;

        if !self.release.is_empty() {
            write!(f, " ({})", self.release)?;
        }
        Ok(())
    }
}
