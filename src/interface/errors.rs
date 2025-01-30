use interzic::models::services::youtube::error::InterzicYoutubeError;
use interzic::models::services::youtube::error::YoutubeError;

pub fn process_errors(error: &crate::Error) -> Option<String> {
    match &error {
        crate::Error::InterzicError(err) => process_interzic_error(err),

        _ => None,
    }
}

fn process_interzic_error(error: &interzic::Error) -> Option<String> {
    match error {
        interzic::Error::YoutubeError(err) => process_interzic_youtube_error(err),
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
