use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::traits::vec_like::VecLike;

impl<Ent, Lis, T> VecLike<T> for EntityWithListens<Ent, Lis>
where
    Lis: VecLike<T>,
{
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.listens.retain(f);
    }
}
