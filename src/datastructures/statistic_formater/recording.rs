use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::ALISTRAL_CLIENT;
use crate::datastructures::statistic_formater::StatisticFormater;

// impl StatisticFormater<Recording> {
//     async fn format_recording_count(
//         &self,
//         rec: &RecordingWithListens,
//     ) -> Result<String, crate::Error> {
//         Ok(format!(
//             "[{}] {}",
//             rec.listen_count(),
//             rec.entity()
//                 .pretty_format_with_credits(
//                     ALISTRAL_CLIENT.musicbrainz_db.get_raw_connection().await?.as_mut(),
//                     ALISTRAL_CLIENT.musicbrainz_db.as_ref(),
//                     true
//                 )
//                 .await?
//         ))
//     }
// }
