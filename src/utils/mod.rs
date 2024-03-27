use color_eyre::owo_colors::OwoColorize;
use derive_builder::Builder;
use listenbrainz::raw::{
    response::{UserListensListen, UserListensResponse},
    Client,
};

pub mod cli_paging;
pub mod extensions;
pub mod traits;

#[derive(Clone, Debug, PartialEq, Eq, Builder)]
#[allow(missing_docs)]
/// Reader for the the User Listens endpoint
pub struct ListenAPIPaginator {
    #[builder(setter(into, strip_option))]
    /// The name of the target user
    user_name: String,

    #[builder(setter(into, strip_option), default)]
    /// The UNIX timestamp of the earliest listen to retreive
    min_ts: Option<i64>,

    #[builder(setter(into, strip_option), default)]
    /// The UNIX timestamp of the latest listen to retreive
    max_ts: Option<i64>,

    #[builder(setter(into, strip_option), default = "Some(999)")]
    /// The number of listen to retreive from the API.
    count: Option<u64>,

    #[builder(setter(into, strip_option), default)]
    time_range: Option<u64>,
}

impl ListenAPIPaginator {
    /// Update [`Self::max_ts`] for the latest listen in the response
    fn update_max_ts(&mut self, responce: &UserListensResponse) {
        self.max_ts = responce
            .payload
            .listens
            .iter()
            .min_by_key(|listen| listen.listened_at)
            .map(|latest_listen| latest_listen.listened_at)
    }

    /// Retreive the next page of listens
    pub fn next(&mut self, client: &Client) -> Result<UserListensResponse, listenbrainz::Error> {
        let response =
            client.user_listens(&self.user_name, self.min_ts, self.max_ts, self.count)?;
        self.update_max_ts(&response);
        Ok(response)
    }

    pub fn into_reader(self) -> ListenAPIReader {
        ListenAPIReader::new(self)
    }
}

pub struct ListenAPIReader {
    paginator: ListenAPIPaginator,
    page: Vec<UserListensListen>,
}

impl ListenAPIReader {
    pub fn new(paginator: ListenAPIPaginator) -> Self {
        Self {
            page: Vec::new(),
            paginator,
        }
    }
}

impl Iterator for ListenAPIReader {
    type Item = UserListensListen;

    fn next(&mut self) -> Option<Self::Item> {
        if self.page.is_empty() {
            let client = Client::new();
            let page = self.paginator.next(&client).unwrap();
            self.page.extend(page.payload.listens);
        }

        self.page.pop()
    }
}

pub fn println_cli(string: &str) {
    println!("{} {}", "[CLI Tools]".green(), string);
}

pub fn println_lis(string: &str) {
    println!("{} {}", "[Listenbrainz]".blue(), string);
}
