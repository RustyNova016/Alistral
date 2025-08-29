pub mod id;
use core::marker::PhantomData;

use itertools::Itertools as _;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::tags::Tag;
use musicbrainz_db_lite::models::shared_traits::has_tags::HasTags;
use musicbrainz_db_lite::utils::sqlx_utils::entity_relations::JoinRelation;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;
use tuillez::pg_spinner;

use crate::AlistralClient;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::tags::id::SimpleTag;
use crate::datastructures::listen_collection::traits::ListenCollectionReadable;
use crate::datastructures::listen_sorter::ListenSortingStrategy;
use crate::traits::mergable::Mergable;

use crate::datastructures::entity_with_listens::EntityWithListens;

pub type TagWithEntListens<Ent, Lis> =
    EntityWithListens<SimpleTag, EntityWithListensCollection<Ent, Lis>>;

pub type TagWithEntListensCollection<Ent, Lis> =
    EntityWithListensCollection<SimpleTag, EntityWithListensCollection<Ent, Lis>>;

pub struct TagWithEntListensStrategy<'l, T, Ent, Lis>
where
    T: ListenSortingStrategy<Ent, Lis>,
    Ent: HasRowID,
    Lis: ListenCollectionReadable,
{
    pub(super) client: &'l AlistralClient,
    ent_strat: T,
    entity: PhantomData<Ent>,
    listens: PhantomData<Lis>,
}

impl<'l, T, Ent, Lis> TagWithEntListensStrategy<'l, T, Ent, Lis>
where
    T: ListenSortingStrategy<Ent, Lis>,
    Ent: HasRowID,
    Lis: ListenCollectionReadable,
{
    pub fn new(client: &'l AlistralClient, ent_strat: T) -> Self {
        Self {
            client,
            ent_strat,
            entity: Default::default(),
            listens: Default::default(),
        }
    }
}

impl<T, Ent, Lis> ListenSortingStrategy<SimpleTag, EntityWithListensCollection<Ent, Lis>>
    for TagWithEntListensStrategy<'_, T, Ent, Lis>
where
    T: ListenSortingStrategy<Ent, Lis>,
    Ent: HasRowID + HasTags + Clone,
    Lis: ListenCollectionReadable + Clone + Mergable,
{
    #[instrument(skip(self, client, data, listens), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn sort_insert_listens(
        &self,
        client: &AlistralClient,
        data: &mut TagWithEntListensCollection<Ent, Lis>,
        listens: Vec<Listen>,
    ) -> Result<(), crate::Error> {
        pg_spinner!("Compiling artist listen data");
        // Convert Recordings
        let listens =
            EntityWithListensCollection::<Ent, Lis>::from_listens(client, listens, &self.ent_strat)
                .await?;

        let entity_refs = listens.iter_entities().collect_vec();
        //fetch_artists_of_recordings(self.client, &recording_refs).await?;

        let conn = &mut *self.client.musicbrainz_db.get_raw_connection().await?;

        // Load artists
        let relations = Tag::query_batch(conn, entity_refs).await?;

        // Convert artists
        compile(data, relations, listens);

        Ok(())
    }

    async fn sort_insert_listen(
        &self,
        client: &AlistralClient,
        data: &mut TagWithEntListensCollection<Ent, Lis>,
        listen: Listen,
    ) -> Result<(), crate::Error> {
        Self::sort_insert_listens(self, client, data, vec![listen]).await
    }
}

#[instrument(skip(data, listens, relations),fields(indicatif.pb_show = tracing::field::Empty))]
fn compile<Ent, Lis>(
    data: &mut TagWithEntListensCollection<Ent, Lis>,
    relations: Vec<JoinRelation<i64, Tag>>,
    listens: EntityWithListensCollection<Ent, Lis>,
) where
    Ent: HasRowID + HasTags + Clone,
    Lis: ListenCollectionReadable + Clone + Mergable,
{
    pg_counted!(relations.len(), "Loading listens data");

    for (entity_id, tag) in relations
        .into_iter()
        .map(|join| (join.original_id, join.data))
    {
        let listen = listens.get_by_id(entity_id);

        if let Some(listen) = listen {
            data.insert_or_merge_entity_stats(TagWithEntListens {
                entity: SimpleTag::new(tag.name.clone()),
                listens: listen.clone().into(),
            });
        }

        pg_inc!();
    }
}
