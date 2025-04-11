use core::marker::PhantomData;

use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;
use futures::stream;
use musicbrainz_db_lite::RowId;

use crate::utils::cli_paging::CLIPager;

pub mod listencount;
pub mod listenduration;

pub struct StatisticFormater<Ent, Lis, Stats>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
    Stats: StatisticType,
{
    data: EntityWithListensCollection<Ent, Lis>,
    stat_type: PhantomData<Stats>,
    #[allow(dead_code)] // Will be used later
    stat_output: StatisticOutput,
}

impl<Ent, Lis, Stats> StatisticFormater<Ent, Lis, Stats>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
    Stats: StatisticType,
{
    pub fn new(data: EntityWithListensCollection<Ent, Lis>, stat_output: StatisticOutput) -> Self {
        Self {
            data,
            stat_type: Default::default(),
            stat_output,
        }
    }
}

impl<Ent, Lis, Stats> StatisticFormater<Ent, Lis, Stats>
where
    Self: StatFormatterVariant<Ent, Lis>,
    Ent: RowId,
    Lis: ListenCollectionReadable,
    Stats: StatisticType,
{
    fn print_stream(&self) -> impl Stream<Item = Result<String, crate::Error>> {
        let data = self.sort_elements();

        stream::iter(data)
            .enumerate()
            .map(async |(i, data)| match self.get_line(data).await {
                Ok(val) => Ok(format!("  {} | {}", i + 1, val)),
                Err(err) => Err(err),
            })
            .buffered(8)
    }

    pub async fn print_paged(&self) -> Result<(), crate::Error> {
        let mut pager = CLIPager::new(10);
        let mut formated = self.print_stream();

        while let Some(line) = formated.try_next().await? {
            println!("{}", line);

            if !pager.inc() {
                break;
            }
        }

        Ok(())
    }
}

// Using a typestate pattern due to the different statistics having different trait bounds
pub struct ListenCountStats;
pub struct ListenDurationStats;

pub trait StatisticType {}
impl StatisticType for ListenCountStats {}
impl StatisticType for ListenDurationStats {}

/// Trait for formatting specific statistic types
pub trait StatFormatterVariant<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
{
    fn sort_elements(&self) -> Vec<&EntityWithListens<Ent, Lis>>;

    async fn get_line(&self, element: &EntityWithListens<Ent, Lis>)
    -> Result<String, crate::Error>;
}

pub enum StatisticOutput {
    Print,
    Art,
}
