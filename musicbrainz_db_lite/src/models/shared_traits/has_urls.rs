use std::sync::Arc;

use crate::DBRelation;
use crate::Url;
use crate::models::shared_traits::db_relation::EntityURLDBRel;

pub trait HasUrls<U>
where
    Self: crate::DBRelationFetch<EntityURLDBRel, U>,
    U: Send,
    Url: From<<Self as DBRelation<EntityURLDBRel>>::ReturnedType>,
{
    /// Return the urls of the entity (Use a task)
    fn get_entity_urls(
        &self,
        client: &Arc<crate::DBClient>,
    ) -> impl std::future::Future<
        Output = Result<Vec<<Self as DBRelation<EntityURLDBRel>>::ReturnedType>, crate::Error>,
    > + Send
    where
        Self: 'static,
        U: 'static,
    {
        async { self.get_related_entity_or_fetch_as_task(client).await }
    }

    /// Return true if the current entity has an url that is on a specific host
    fn has_url_with_host(
        &self,
        client: &Arc<crate::DBClient>,
        host: &url::Host<&str>,
    ) -> impl std::future::Future<Output = Result<Option<Url>, crate::Error>> + Send
    where
        Self: 'static,
        U: 'static,
    {
        async {
            let urls = self.get_entity_urls(client).await?;

            Ok(urls
                .into_iter()
                .map(Url::from)
                .find(|url| url.match_host(host)))
        }
    }
}

impl<T, U> HasUrls<U> for T
where
    T: crate::DBRelationFetch<EntityURLDBRel, U>,
    U: Send,
    Url: From<<T as DBRelation<EntityURLDBRel>>::ReturnedType>,
{
}
