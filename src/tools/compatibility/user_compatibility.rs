use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use rust_decimal::Decimal;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

//TODO: #459 Refactor user compatibility with a "UserWithListens" struct

/// Return a list of recordings that are both listened by user A and user B
pub fn get_shared_recordings_between_users(
    user_a_recordings: &RecordingWithListensCollection,
    user_b_recordings: &RecordingWithListensCollection,
) -> Vec<Recording> {
    let mut recordings = Vec::new();

    for recording_a in user_a_recordings.iter_entities() {
        for recording_b in user_b_recordings.iter_entities() {
            if recording_a == recording_b {
                recordings.push(recording_a.clone());
            }
        }
    }

    recordings
}

/// Return the percentage of listens of user A that are also listened by user B
pub fn get_user_shared_percent(
    shared_recordings: &[Recording],
    user_recordings: &RecordingWithListensCollection,
) -> Decimal {
    Decimal::new(shared_recordings.len().try_into().unwrap(), 0)
        / Decimal::new(user_recordings.listen_count().try_into().unwrap(), 0)
        * Decimal::ONE_HUNDRED
}

/// For each shared recordings, return the ratio of listens being from a recording
#[instrument( fields(indicatif.pb_show = tracing::field::Empty))]
fn get_user_ratio<'r>(
    shared_recordings: &'r Vec<Recording>,
    user_listens: &RecordingWithListensCollection,
) -> Vec<(Decimal, &'r Recording)> {
    pg_counted!(shared_recordings.len(), "Calculating listen ratios");

    let mut ratios = Vec::new();
    for shared_rec in shared_recordings {
        ratios.push((user_listens.get_listen_ratio(shared_rec), shared_rec));
        pg_inc!();
    }

    ratios
}

pub fn get_shared_ratio(
    shared_recordings: &Vec<Recording>,
    user_a_listens: &RecordingWithListensCollection,
    user_b_listens: &RecordingWithListensCollection,
) -> Decimal {
    let mut total_ratio = Decimal::ZERO;
    let ratios_a = get_user_ratio(shared_recordings, user_a_listens);
    let ratios_b = get_user_ratio(shared_recordings, user_b_listens);

    for shared_recording in shared_recordings {
        let Some((ratio_a, _)) = ratios_a.iter().find(|(_, rec)| *rec == shared_recording) else {
            continue;
        };
        let Some((ratio_b, _)) = ratios_b.iter().find(|(_, rec)| *rec == shared_recording) else {
            continue;
        };

        if ratio_a < ratio_b {
            total_ratio += ratio_a;
        } else {
            total_ratio += ratio_b;
        }
    }

    total_ratio
}
