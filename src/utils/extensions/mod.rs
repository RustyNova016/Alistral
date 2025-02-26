use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use extend::ext;
use listenbrainz::raw::response::{UserListensListen, UserListensMBIDMapping, UserListensPayload};

pub mod chrono_ext;
pub mod listenbrainz_ext;

#[ext]
pub impl UserListensPayload {
    /// Return the oldest
    fn get_date_of_oldest_listen_of_user(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.oldest_listen_ts, 0)
            .single()
            .expect("Error: Could not parse listen's timestamp")
    }

    fn get_date_of_latest_listen_of_user(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.latest_listen_ts, 0)
            .single()
            .expect("Error: Could not parse listen's timestamp")
    }

    fn get_oldest_payload_listen(&self) -> Option<&UserListensListen> {
        self.listens.iter().min_by_key(|listen| listen.listened_at)
    }

    fn get_latest_payload_listen(&self) -> Option<&UserListensListen> {
        self.listens.iter().max_by_key(|listen| listen.listened_at)
    }

    fn get_date_of_oldest_listen_of_payload(&self) -> Option<DateTime<Utc>> {
        self.get_oldest_payload_listen()
            .and_then(|listen| Utc.timestamp_opt(listen.listened_at, 0).single())
    }

    fn get_date_of_latest_listen_of_payload(&self) -> Option<DateTime<Utc>> {
        self.get_latest_payload_listen()
            .and_then(|listen| Utc.timestamp_opt(listen.listened_at, 0).single())
    }
}

#[ext]
pub impl UserListensListen {
    fn get_listened_at_date(&self) -> DateTime<Utc> {
        Utc.timestamp_opt(self.listened_at, 0)
            .single()
            .expect("Error: Could not parse listen's timestamp")
    }
}

#[ext]
pub impl UserListensMBIDMapping {
    /// Return the artist credit as a string from the artist name and join phrases
    fn get_artist_credit_as_string(&self) -> Option<String> {
        let artist_credits = self.artists.as_ref()?;

        let mut credit_string = String::new();
        for artist_credit in artist_credits {
            credit_string.push_str(&artist_credit.artist_credit_name);
            credit_string.push_str(&artist_credit.join_phrase);
        }

        Some(credit_string)
    }
}
