/// Configuration of the daily command
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "docs", derive(schemars::JsonSchema))]
pub struct DailyCommandConfig {
    /// The minimum listens needed to display a track's anniversary / first discovery
    pub minimum_listens: Option<usize>,
}
