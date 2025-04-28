// macro_rules! impl_fetch_by_mbid {
//     ($row_struct: ty) => {
//         impl $row_struct {
//             pub async fn find_by_mbid(
//                 conn: &mut sqlx::SqliteConnection,
//                 mbid: &str,
//             ) -> Result<Option<$row_struct>, sqlx::Error> {
//                 <Self as $crate::MBIDRedirection>::find_by_mbid(conn, mbid)
//             }
//         }
//     };
// }

// pub(crate) use impl_fetch_by_mbid;
