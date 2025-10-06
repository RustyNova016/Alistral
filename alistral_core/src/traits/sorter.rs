pub trait Linker<V, I> {
    type Returned;

    fn link_entry(
        &self,
        data: &mut V,
        item: I,
    ) -> impl std::future::Future<Output = Self::Returned> + Send;

    fn link_entry_batch(
        &self,
        data: &mut V,
        items: Vec<I>,
    ) -> impl std::future::Future<Output = Self::Returned> + Send;
}

pub trait InsertElement<L, I>
where
    L: Linker<Self, I> + Sync,
    I: Send,
    Self: Sized + Send,
{
    fn insert_element(
        &mut self,
        linker: &L,
        item: I,
    ) -> impl std::future::Future<Output = L::Returned> + Send {
        async { linker.link_entry(self, item).await }
    }

    fn insert_elements(
        &mut self,
        linker: &L,
        item: Vec<I>,
    ) -> impl std::future::Future<Output = L::Returned> + Send {
        async { linker.link_entry_batch(self, item).await }
    }
}

impl<L, I, T> InsertElement<L, I> for T
where
    L: Linker<Self, I> + Sync,
    I: Send,
    Self: Sized + Send,
{
}
