// use chrono::DateTime;
// use chrono::Utc;

// use crate::AlistralClient;
// use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;

// impl RecordingWithListensCollection {
//     pub async fn filter_by_release_date(&mut self, client: &AlistralClient, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<(), crate::Error>{
//         for (id, rec) in self.0.iter_mut() {
//             let rec_release = rec.entity().first_release_date_or_fetch(client.musicbrainz_db.clone()).await?;

//             if rec_release.is_none_or(|rel| rel < from) && rec_release.is_none_or(|rel| rel > to) {
//                 self.0.remove(id);
//             } 
//         }

//         Ok(())
//     }
// }