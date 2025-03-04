use extend::ext;
use listenbrainz::raw::response::UserListensTrackMetadata;

#[ext]
pub impl UserListensTrackMetadata {
    fn get_additional_string_metadata(&self, key: &str) -> Option<&String> {
        let data = self.additional_info.get(key)?;
        match data {
            serde_json::Value::String(data) => Some(data),
            _ => None,
        }
    }
}
