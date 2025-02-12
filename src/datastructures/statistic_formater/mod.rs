use core::marker::PhantomData;

use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::listen_collection::ListenCollection;
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;
use futures::stream;
use musicbrainz_db_lite::RowId;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::utils::cli_paging::CLIPager;

pub mod listencount;
pub mod recording;

pub struct StatisticFormater<T, Stats>
where
    T: RowId,
    Stats: StatisticType,
{
    data: EntityWithListensCollection<T, ListenCollection>,
    stat_type: PhantomData<Stats>,
    stat_output: StatisticOutput,
}

impl<T, Stats> StatisticFormater<T, Stats>
where
    T: RowId,
    Stats: StatisticType,
{
    pub fn new(
        data: EntityWithListensCollection<T, ListenCollection>,
        stat_output: StatisticOutput,
    ) -> Self {
        Self {
            data,
            stat_type: Default::default(),
            stat_output,
        }
    }
}

impl<T, Stats> StatisticFormater<T, Stats>
where
    Self: StatFormatterVariant<T>,
    T: RowId,
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

impl StatisticFormater<Recording, ListenCountStats> {
    async fn a(&self) {
        self.print_paged().await;
    }
}

// Using a typestate pattern due to the different statistics having different trait bounds
pub struct ListenCountStats;
pub struct ListenDurationStats;

pub trait StatisticType {}
impl StatisticType for ListenCountStats {}
impl StatisticType for ListenDurationStats {}

pub trait StatFormatterVariant<T>
where
    T: RowId,
{
    fn sort_elements(&self) -> Vec<&EntityWithListens<T, ListenCollection>>;

    async fn get_line(
        &self,
        element: &EntityWithListens<T, ListenCollection>,
    ) -> Result<String, crate::Error>;
}

pub enum StatisticOutput {
    Print,
    Art,
}
