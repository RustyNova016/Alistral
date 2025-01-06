pub mod recording;
use core::marker::PhantomData;

pub mod lookup_trait;
pub struct Lookup<Target, Data, Type> {
    /// The target entity on which we lookup the information
    target: Target,

    current_timerange: Data,
    previous_timerange: Option<Data>,
    phantom_type: PhantomData<Type>
}
