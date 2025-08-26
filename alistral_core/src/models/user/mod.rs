use musicbrainz_db_lite::User;

use crate::AlistralClient;
use crate::database::fetching::listens::ListenFetchQuery;
use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::user::UserWithListens;
use crate::datastructures::entity_with_listens::user::collection::UserWithListensStrategy;
use crate::datastructures::listen_collection::ListenCollection;

pub struct UserData {
    listens: UserWithListens,

    recordings: Option<RecordingWithListensCollection>,
}

impl UserData {
    pub async fn load_user(client: &AlistralClient, name: String) -> Result<Self, crate::Error> {
        let strat = UserWithListensStrategy::new(&client);
        Self::load_user_with_strat(client, name, &strat).await
    }

    pub async fn load_user_with_strat<'l>(
        client: &AlistralClient,
        name: String,
        strat: &UserWithListensStrategy<'l>,
    ) -> Result<Self, crate::Error> {
        let listens =
            ListenFetchQuery::get_entity_with_listens(client, name.clone(), strat).await?;

        Ok(Self {
            listens: listens
                .into_iter()
                .find(|user| user.entity().name.to_lowercase() == name.to_lowercase())
                .unwrap_or_else(|| {
                    let user = User { id: 0, name };
                    EntityWithListens::new(user, ListenCollection::default())
                }),
            recordings: None,
        })
    }

    pub fn user_with_listens(&self) -> &UserWithListens {
        &self.listens
    }

    /// Provide the recording stats for the user as a mutable reference
    pub async fn recording_stats_mut(
        &mut self,
        client: &AlistralClient,
    ) -> Result<&mut RecordingWithListensCollection, crate::Error> {
        if self.recordings.is_some() {
            return Ok(self.recordings.as_mut().unwrap());
        }

        let data = RecordingWithListensCollection::from_listencollection(
            client,
            self.listens.listens().to_owned(),
            &client.recording_with_listen_strat,
        )
        .await?;

        self.recordings.replace(data);
        Ok(self.recordings.as_mut().unwrap())
    }

    /// Provide the recording stats for the user
    pub async fn recording_stats(
        &mut self,
        client: &AlistralClient,
    ) -> Result<&RecordingWithListensCollection, crate::Error> {
        self.recording_stats_mut(client).await.map(|r| &*r)
    }
}
