use chrono::Utc;
use sequelles::databases::sqlite::database::GetConnectionError;
use snafu::Location;
use snafu::ResultExt;

use crate::DBClient;
use crate::error::sqlx_error::SqlxError;
use crate::error::sqlx_error::SqlxSnafu;
use crate::models::listenbrainz::listen::Listen;

#[derive(Debug, bon::Builder)]
pub struct AdvancedListenQuery {
    /// Filter to only listens of those users
    #[builder(default)]
    users: Vec<String>,

    /// Fetch listens after this timestamps (inclusive)
    #[builder(default = 0)]
    min_ts: i64,

    /// Fetch listens before this timestamps (inclusive)
    #[builder(default = Utc::now().timestamp())]
    max_ts: i64,

    /// Fetch listens that are linked
    #[builder(default = true)]
    unlinked: bool,

    /// Fetch listens that are unlinked
    #[builder(default = true)]
    linked: bool,
}

impl AdvancedListenQuery {
    pub async fn fetch(&self, client: &DBClient) -> Result<Vec<Listen>, AdvancedListenQueryError> {
        let mut linking_rule = String::new();

        if self.linked {
            linking_rule.extend("listen.recording_msid IN (SELECT recording_msid FROM msid_mapping WHERE recording_msid = listen.recording_msid) OR".chars());
        }

        if self.unlinked {
            linking_rule.extend("listen.recording_msid NOT IN (SELECT recording_msid FROM msid_mapping WHERE recording_msid = listen.recording_msid) OR".chars());
        }

        Ok(sqlx::query_as(&format!(
            "
                    SELECT listens.* 
                    FROM listens 
                    WHERE 
                        listen.user IN (SELECT value FROM JSON_EACH($1))
                        AND listen.listened_at >= $2 
                        AND listen.listened_at <= $3 
                        AND ({linking_rule} 1 = 0)"
        ))
        .bind(serde_json::to_string(&self.users).context(SerdeSnafu)?)
        .bind(self.min_ts)
        .bind(self.max_ts)
        .fetch_all(&mut *client.get_conn().await?)
        .await.context(SqlxSnafu)?)
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum AdvancedListenQueryError {
    #[snafu(display("Couldn't serialize the data for the sql query"))]
    SerdeError {
        source: serde_json::Error,

        #[snafu(implicit)]
        location: Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    #[snafu(transparent)]
    ConnectionError {
        #[cfg_attr(feature = "backtrace", snafu(backtrace))]
        source: GetConnectionError,

        #[snafu(implicit)]
        location: Location,
    },

    #[snafu(transparent)]
    SqlxError {
        #[cfg_attr(feature = "backtrace", snafu(backtrace))]
        source: SqlxError,

        #[snafu(implicit)]
        location: Location,
    },
}
