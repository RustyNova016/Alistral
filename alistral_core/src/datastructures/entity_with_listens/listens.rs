use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::listen_collection::ListenCollection;

impl<Ent, Lis> EntityWithListens<Ent, Lis> {
    pub fn listens(&self) -> &Lis {
        &self.listens
    }

    pub fn into_listens(self) -> Lis {
        self.listens
    }

    /// Set the listens in the collection.
    pub fn set_listens(&mut self, listens: Lis) {
        self.listens = listens
    }
}

impl<Ent> EntityWithListens<Ent, ListenCollection> {
    /// Add a listen if it doesn't already exist in the collection. This doesn't check if the listen belong to the entity
    pub fn insert_unique_listen_unchecked(&mut self, new_listen: Listen) {
        self.listens.push_unique(new_listen);
    }

    /// Add a collection of listen if it doesn't already exist in the collection. This doesn't check if the listen belong to the entity
    pub fn insert_unique_listens_unchecked<I: IntoIterator<Item = Listen>>(
        &mut self,
        new_listens: I,
    ) {
        for lis in new_listens {
            self.listens.push_unique(lis);
        }
    }
}
