use chrono::DateTime;
use chrono::TimeZone;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, bon::Builder)]
pub struct FreshReleaseRequest<Tz>
where
    Tz: TimeZone,
{
    release_date: DateTime<Tz>,
    days: u8,
    past: bool,
    future: bool,
}

impl<Tz> FreshReleaseRequest<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: std::fmt::Display,
{
    pub fn get_parameters(&self) -> String {
        format!(
            "?release_date={}&days={}&past={}&future={}",
            self.release_date.format("%Y-%m-%d"),
            self.days,
            self.past,
            self.future
        )
    }

    pub async fn fetch(&self) -> Result<FreshReleaseResponse, crate::Error> {
        let response = reqwest::get(format!(
            "https://api.listenbrainz.org/1/explore/fresh-releases/{}",
            self.get_parameters()
        ))
        .await?;

        Ok(response.json().await?)
    }
}

impl<Tz> Default for FreshReleaseRequest<Tz>
where
    Tz: TimeZone,
    chrono::DateTime<Tz>: std::default::Default,
{
    fn default() -> Self {
        Self {
            days: 90,
            future: true,
            past: true,

            release_date: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FreshReleaseResponse {
    pub payload: FreshReleasePayload,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FreshReleasePayload {
    pub releases: Vec<FreshReleaseRelease>,
    pub total_count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FreshReleaseRelease {
    pub artist_credit_name: String,
    pub artist_mbids: Vec<String>,

    pub release_date: String,
    pub release_group_mbid: String,
    pub release_group_primary_type: Option<String>,
    pub release_mbid: String,
    pub release_name: String,
}
