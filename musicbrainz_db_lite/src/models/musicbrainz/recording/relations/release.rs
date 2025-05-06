use crate::DBRelation;
use crate::Recording;
use crate::Release;

/// Recording (1:M) -> Releases
pub struct RecordingReleasesDBRel;

impl DBRelation<RecordingReleasesDBRel> for Recording {
    type ReturnedType = Release;

    fn get_join_statement() -> &'static str {
        "INNER JOIN tracks ON recordings.id = tracks.recording
        INNER JOIN medias ON tracks.media = medias.id
        INNER JOIN releases ON medias.`release` = releases.id"
    }
}