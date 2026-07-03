pub trait OrderWith<T>: Sized {
    // fn order_with(mut self, orderer: T) -> Self {
    //     self.order_with_mut(orderer);
    //     self
    // }

    fn order_with_mut(&mut self, orderer: T) -> &mut Self;
}
