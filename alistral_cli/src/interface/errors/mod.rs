pub mod friendly_error;

pub fn process_errors(error: &crate::Error) -> Option<String> {
    match &error {
        #[cfg(feature = "interzic")]
        crate::Error::Listenbrainz(err) => process_listenbrainz_error(err),
        _ => None,
    }
}

fn process_listenbrainz_error(error: &listenbrainz::Error) -> Option<String> {
    match error {
        listenbrainz::Error::Api { error, .. } => {
            if error == "Invalid authorization token." {
                return Some("The authentification token is invalid.".to_string());
            }

            None
        }
        _ => None,
    }
}
