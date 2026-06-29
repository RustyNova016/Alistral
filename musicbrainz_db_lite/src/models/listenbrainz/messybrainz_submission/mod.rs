use sequelles::has_rowid::HasRowID;
use sqlx::prelude::FromRow;

pub mod finds;
pub mod relations;
pub mod selects;

/// The fingerprint that identify a listened recording. This is the data scrobblers send to LB to tell that the user listened to a recording
#[derive(Debug, FromRow, Clone, sequelles::Table)]
#[sequelles(db_name = "messybrainz_submission", snafu)]
#[sequelles(sqlite)]
#[sequelles(upsert, insert, insert_struct)]
#[sequelles(primary_key(key_name = "pk", columns(id)))]
#[sequelles(unique_key(key_name = "msid", columns(msid)))]
pub struct MessybrainzSubmission {
    #[sequelles(auto_increment)]
    pub id: i64,
    pub msid: String,
    pub recording: String,
    pub artist_credit: String,
    pub release: Option<String>,
    pub track_number: Option<String>,
    pub duration: Option<i64>,
}

impl HasRowID for MessybrainzSubmission {
    fn rowid(&self) -> i64 {
        self.id
    }
}
