use futures::stream::BoxStream;

use crate::radio_item::RadioItem;

pub type LayerResult<'a> = Result<RadioStream<'a>, crate::Error>;

pub type RadioResult = Result<RadioItem, crate::Error>;

pub type RadioStream<'a> = BoxStream<'a, RadioResult>;
