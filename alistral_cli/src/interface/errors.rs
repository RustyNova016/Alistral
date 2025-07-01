use interzic::models::services::youtube::error::InterzicYoutubeError;
use interzic::models::services::youtube::error::YoutubeError;

pub fn process_errors(error: &crate::Error) -> Option<String> {
    match &error {
        crate::Error::Interzic(err) => process_interzic_error(err),
        crate::Error::Listenbrainz(err) => process_listenbrainz_error(err),
        _ => None,
    }
}

fn process_interzic_error(error: &interzic::Error) -> Option<String> {
    match error {
        interzic::Error::YoutubeError(err) => process_interzic_youtube_error(err),
        _ => None,
    }
}

fn process_listenbrainz_error(error: &listenbrainz::Error) -> Option<String> {
    match error {
        listenbrainz::Error::Api { code: _, error } => {
            if error == "Invalid authorization token." {
                return Some("The authentification token is invalid.".to_string());
            }

            None
        }
        _ => None,
    }
}

fn process_interzic_youtube_error(error: &InterzicYoutubeError) -> Option<String> {
    #[expect(
        clippy::match_single_binding,
        reason = "This will get expanded later, so let's keep it ready"
    )]
    match error {
        _ => error.as_youtube_error().and_then(process_youtube_error),
    }
}

fn process_youtube_error(error: &YoutubeError) -> Option<String> {
    match error {
        YoutubeError::QuotaExceededError(_) => Some("The quota of your Youtube application (youtube api v3) is fully spent. Please wait until it comes back".to_string()),
        _ => None,
    }
}
