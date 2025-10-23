use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;

use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsyncDyn;
use tuillez::reexports::async_trait;

pub(crate) static TAG_IDS: LazyLock<Arc<Mutex<HashMap<String, i64>>>> =
    LazyLock::new(Default::default);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SimpleTag {
    id: i64,
    name: String,
}

impl SimpleTag {
    pub fn new(name: String) -> Self {
        let mut ids = TAG_IDS.lock().unwrap();
        let id = match ids.get(&name) {
            Some(id) => *id,
            None => {
                let id = ids.len().try_into().unwrap();
                ids.insert(name.clone(), id);
                id
            }
        };

        Self { id, name }
    }
}

impl HasRowID for SimpleTag {
    fn rowid(&self) -> i64 {
        self.id
    }
}

#[async_trait]
impl FormatWithAsyncDyn<MusicbrainzFormater> for SimpleTag {
    type Error = crate::Error;

    async fn format_with_async(&self, _ft: &MusicbrainzFormater) -> Result<String, Self::Error> {
        Ok(self.name.to_string())
    }
}
