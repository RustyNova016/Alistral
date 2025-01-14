use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::datastructures::listen_collection::traits::ListenCollectionLike;

use super::collection::RecordingWithListensCollection;
use super::RecordingWithListens;

impl RecordingWithListens {
    pub fn get_underated_score(
        &self,
        user_listens: &RecordingWithListensCollection,
        global_listen_count: u64,
    ) -> Decimal {
        self.get_underrated_rank_score(user_listens)
            + self.get_underrated_listen_score(user_listens, global_listen_count)
    }

    fn get_underrated_rank_score(&self, user_listens: &RecordingWithListensCollection) -> Decimal {
        // Retrive the rank of the recording in the user listens
        let rank = user_listens.get_rank(&self.recording.mbid).unwrap_or(9999);

        // The recording vec scores if its between 0 and 999.
        Decimal::from(1000_u64.saturating_sub(rank.try_into().unwrap()))
            .checked_div(dec!(10))
            .unwrap_or(Decimal::ZERO)
    }

    fn get_underrated_listen_score(
        &self,
        user_listens: &RecordingWithListensCollection,
        global_listen_count: u64,
    ) -> Decimal {
        // Retrieve the all time listen count of the recording for the user
        let all_time_listen_count = user_listens
            .get_by_mbid(&self.recording.mbid)
            .map(|r| r.listen_count())
            .unwrap_or(0);

        // Divide by the global listen count to get the fractions of listens made by the user
        // Then x100 to get the score to max out at 100 points
        Decimal::from(all_time_listen_count)
            .checked_div(Decimal::from(global_listen_count))
            // If `global_listen_count` is 0, it means that the stats haven't been processed yet on LB's side.
            // We'll be optimist and assume the user is the only listener
            .unwrap_or(dec!(1))
            // if `global_listen_count` is inferior to the stats, we cap at one.
            .min(Decimal::ONE)
            .saturating_mul(dec!(100))
    }
}

// #[cfg(test)]
// mod tests {
//     use rust_decimal::Decimal;

//     use crate::testing::fixtures::user_listens::get_test_user_recording_with_listens;

//     #[tokio::test]
//     #[serial_test::serial]
//     async fn underrated_score() {
//         let user_listens = get_test_user_recording_with_listens().await;
//         let tests_recordings = vec![
//             ("5fed738b-1e5c-4a1b-9f66-b3fd15dbc8ef", 100), // Top track
//         ];

//         for (test_id, result) in tests_recordings {
//             let data = user_listens
//                 .get_by_mbid(test_id)
//                 .expect("The recording should exist");
//             let score = data.get_underrated_rank_score(&user_listens);

//             assert_eq!(score, Decimal::from(result));
//         }
//     }
// }
