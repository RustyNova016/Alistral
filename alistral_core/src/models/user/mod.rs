use itertools::Itertools;
use musicbrainz_db_lite::User;

use crate::AlistralClient;
use crate::database::fetching::listens::ListenFetchQuery;
use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListenStrategy;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::user::UserWithListens;
use crate::datastructures::entity_with_listens::user::collection::UserWithListensStrategy;
use crate::datastructures::listen_collection::ListenCollection;
use crate::datastructures::listen_sorter::ListenSortingStrategy as _;

pub struct UserData {
    listens: UserWithListens,
}

impl UserData {
    pub async fn load_user<'l>(
        client: &AlistralClient,
        name: String,
    ) -> Result<Self, crate::Error> {
        let strat = UserWithListensStrategy::new(&client);
        Self::load_user_with_strat(&client, name, &strat).await
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
        })
    }

    pub fn user_with_listens(&self) -> &UserWithListens {
        &self.listens
    }

    pub async fn recordings_with_listens(
        &self,
        client: &AlistralClient,
    ) -> Result<RecordingWithListensCollection, crate::Error> {
        let strat = RecordingWithListenStrategy::new(client);
        self.recordings_with_listens_with_strat(strat).await
    }

    pub async fn recordings_with_listens_with_strat(
        &self,
        strat: RecordingWithListenStrategy<'_>,
    ) -> Result<RecordingWithListensCollection, crate::Error> {
        let listens = self.listens.listens().iter().cloned().collect_vec();
        let mut data = RecordingWithListensCollection::default();
        strat.sort_insert_listens(&mut data, listens).await?;
        Ok(data)
    }
}
