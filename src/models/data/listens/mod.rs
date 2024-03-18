use chrono::{DateTime, TimeZone, Utc};
use listenbrainz::raw::response::{UserListensListen, UserListensMBIDMapping};

pub mod collection;

pub struct UserListen {
    /// Time of when the listen happened
    listened_at: DateTime<Utc>,

    /// Data that have been sent to listenbrainz durring listen submition
    messybrainz_data: MessyBrainzData,

    /// Data of the mapping
    mapping_data: Option<MappingData>,
}

impl UserListen {
    pub fn is_mapped(&self) -> bool {
        self.mapping_data.is_some()
    }
}

impl TryFrom<UserListensListen> for UserListen {
    type Error = &'static str;

    fn try_from(value: UserListensListen) -> Result<Self, Self::Error> {
        let listened_at = Utc
            .timestamp_opt(value.listened_at, 0)
            .single()
            .ok_or("Cannot convert listened_at timestamp")?;

        Ok(Self {
            listened_at,
            messybrainz_data: MessyBrainzData::from(value.clone()),
            mapping_data: value
                .track_metadata
                .mbid_mapping
                .map(MappingData::from),
        })
    }
}

pub struct MessyBrainzData {}

impl From<UserListensListen> for MessyBrainzData {
    fn from(_value: UserListensListen) -> Self {
        Self {}
    }
}

pub struct MappingData {
    /// The MBID of the recordings
    recording_mbid: String,
}

impl From<UserListensMBIDMapping> for MappingData {
    fn from(value: UserListensMBIDMapping) -> Self {
        Self {
            recording_mbid: value.recording_mbid,
        }
    }
}