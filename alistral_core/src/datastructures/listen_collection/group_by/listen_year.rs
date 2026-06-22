use std::collections::HashMap;

use chrono::Datelike;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::datastructures::listen_collection::ListenCollection;

impl ListenCollection {
    pub fn group_by_listen_year(self) -> HashMap<i32, Vec<Listen>> {
        let mut map: HashMap<i32, Vec<Listen>> = HashMap::new();

        for listen in self.data {
            map.entry(listen.listened_at_as_datetime().year())
                .or_default()
                .push(listen);
        }

        map
    }
}
