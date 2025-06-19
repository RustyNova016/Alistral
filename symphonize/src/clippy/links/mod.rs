use musicbrainz_db_lite::MainEntity;

use crate::SymphonyzeClient;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::utils::harmony::get_harmony_compatible_release_for_recording;

pub(crate) async fn create_harmony_action_link(
    client: &SymphonyzeClient,
    entity: MainEntity,
) -> Result<Option<MbClippyLintLink>, crate::Error> {
    let release = match entity {
        MainEntity::Release(release) => release,
        MainEntity::Recording(recording) => {
            match get_harmony_compatible_release_for_recording(client, &recording).await? {
                Some(release) => release,
                None => return Ok(None),
            }
        }
        _ => {
            debug_assert!(
                false,
                "Tried to get harmony_action_lint for an incompatible entity"
            );
            return Ok(None);
        }
    };

    Ok(Some(MbClippyLintLink {
        name: "Harmony release actions".to_string(),
        url: format!(
            "https://harmony.pulsewidth.org.uk/release/actions?release_mbid={}",
            release.mbid
        ),
    }))
}
