pub mod listen_count;
pub mod listen_duration;

/// Trait to order items in a vec
pub trait Orderer<T> {
    fn order(&self, items: Vec<T>) -> Vec<T>;
}
