use crate::models::messy_recording::MessyRecording;
use crate::InterzicClient;

#[derive(Debug, Clone)]
pub struct PlaylistStub {
    pub title: String,
    pub description: String,
    pub recordings: Vec<MessyRecording>,
    //TODO: #521 Allow setting playlist visibility
}

impl PlaylistStub {
    pub async fn save_recordings(self, client: &InterzicClient) -> Result<Self, crate::Error> {
        let mut saved_recordings = Vec::new();

        for rec in self.recordings {
            saved_recordings.push(rec.upsert(&client.database_client).await?);
        }

        Ok(Self {
            title: self.title,
            description: self.description,
            recordings: saved_recordings,
        })
    }
}
