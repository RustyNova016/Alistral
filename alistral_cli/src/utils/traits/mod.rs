pub trait VecWrapper<T> {
    fn get_vec(&self) -> &Vec<T>;

    fn len(&self) -> usize {
        self.get_vec().len()
    }

    fn first(&self) -> Option<&T> {
        self.get_vec().first()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
