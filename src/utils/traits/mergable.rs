pub trait Mergable {
    fn merge(&mut self, other: Self);
}
