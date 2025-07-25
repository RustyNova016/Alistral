use musicbrainz_db_lite::User;

use crate::AlistralClient;
use crate::database::fetching::listens::ListenFetchQuery;
use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::entity_with_listens::user::UserWithListens;
use crate::datastructures::entity_with_listens::user::collection::UserWithListensStrategy;
use crate::datastructures::listen_collection::ListenCollection;

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
}
