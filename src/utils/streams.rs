use futures::Stream;
use futures::StreamExt;
use futures::stream;

/// Turns an hard type variable into an opaque type
pub fn into_opaque_stream<T, U>(val: T) -> impl Stream<Item = U>
where
    T: Stream<Item = U> + StreamExt,
{
    val.map(|v| v)
}

pub fn stream_iter_opaque<T: IntoIterator<Item = U>, U>(val: T) -> impl Stream<Item = U> {
    stream::iter(val)
}
