pub trait VecLike<T> {
    /// Keep only the elements that returns true
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool;
}

impl<T> VecLike<T> for Vec<T> {
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.retain(f);
    }
}
