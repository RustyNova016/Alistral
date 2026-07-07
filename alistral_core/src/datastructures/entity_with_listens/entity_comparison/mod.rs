use musicbrainz_db_lite::HasRowID;

use crate::datastructures::entity_with_listens::EntityWithListens;

#[derive(Debug, Clone, Default)]
pub struct EntityListensComparison<Ent, Lis> {
    current: Option<EntityWithListens<Ent, Lis>>,
    previous: Option<EntityWithListens<Ent, Lis>>,
}

impl<Ent, Lis> EntityListensComparison<Ent, Lis> {
    pub fn new(
        current: Option<EntityWithListens<Ent, Lis>>,
        previous: Option<EntityWithListens<Ent, Lis>>,
    ) -> Self
    where
        Ent: HasRowID,
        Self: Default,
    {
        let mut this = Self::default();

        if let Some(val) = current {
            this.set_current(val)
        }

        if let Some(val) = previous {
            this.set_previous(val)
        }

        this
    }

    pub fn set_current(&mut self, current: EntityWithListens<Ent, Lis>)
    where
        Ent: HasRowID,
    {
        if self
            .previous
            .as_ref()
            .is_some_and(|prev| prev.entity().rowid() != current.entity().rowid())
        {
            return;
        }

        self.current = Some(current)
    }

    pub fn set_previous(&mut self, previous: EntityWithListens<Ent, Lis>)
    where
        Ent: HasRowID,
    {
        if self
            .current
            .as_ref()
            .is_some_and(|curr| curr.entity().rowid() != previous.entity().rowid())
        {
            return;
        }

        self.previous = Some(previous)
    }

    pub fn entity(&self) -> Option<&Ent> {
        self.current
            .as_ref()
            .map(|cur| cur.entity())
            .or_else(|| self.previous.as_ref().map(|prev| prev.entity()))
    }

    /// Get the current data from the struct. If it doesn't exist, create an empty one using the previous's entity. If the struct is none, return none
    pub fn current_or_empty(&self) -> Option<EntityWithListens<Ent, Lis>>
    where
        Ent: Clone,
        Lis: Default,
        EntityWithListens<Ent, Lis>: Clone,
    {
        match (&self.current, &self.previous) {
            (Some(val), _) => Some(val.clone()),
            (None, Some(val)) => Some(EntityWithListens::new(
                val.entity().clone(),
                Default::default(),
            )),
            (None, None) => None,
        }
    }

    /// Get the previous data from the struct. If it doesn't exist, create an empty one using the current's entity. If the struct is none, return none
    pub fn previous_or_empty(&self) -> Option<EntityWithListens<Ent, Lis>>
    where
        Ent: Clone,
        Lis: Default,
        EntityWithListens<Ent, Lis>: Clone,
    {
        match (&self.current, &self.previous) {
            (_, Some(val)) => Some(val.clone()),
            (Some(val), None) => Some(EntityWithListens::new(
                val.entity().clone(),
                Default::default(),
            )),
            (None, None) => None,
        }
    }
}
