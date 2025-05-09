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

/// Convenience methods to use the [`crate::DBRelation`] trait without import and type specification
macro_rules! impl_db_relation_methods {
    ($row_struct: ty) => {
        impl $row_struct {
            /// Get the related entities based on the relation type given in the type parameter
            pub async fn get_related_entity<T>(
                &self,
                conn: &mut sqlx::SqliteConnection,
            ) -> Result<Vec<<Self as $crate::DBRelation<T>>::ReturnedType>, crate::Error>
            where
                Self: $crate::DBRelation<T>,
            {
                <Self as $crate::DBRelation<T>>::get_related_entity(&self, conn).await
            }

            /// Get the related entities in bulk based on the relation type given in the type parameter
            pub async fn get_related_entity_bulk<T>(
                conn: &mut sqlx::SqliteConnection,
                entities: &[&Self],
            ) -> Result<
                sequelles::JoinCollection<<Self as $crate::DBRelation<T>>::ReturnedType>,
                crate::Error,
            >
            where
                Self: $crate::DBRelation<T>,
            {
                <Self as $crate::DBRelation<T>>::get_related_entity_bulk(conn, entities).await
            }
        }
    };
}
pub(crate) use impl_db_relation_methods;

/// Convenience methods to use the [`crate::DBRelationFetch`] trait without import and type specification
macro_rules! impl_db_relation_fetch_methods {
    ($row_struct: ty, $u_type: ty) => {
        impl $row_struct {
            /// Get the related entities based on the relation type given in the type parameter
            pub async fn get_related_entity_or_fetch_with_conn<T>(
                &self,
                conn: &mut sqlx::SqliteConnection,
                client: &$crate::DBClient,
            ) -> Result<Vec<<Self as $crate::DBRelation<T>>::ReturnedType>, crate::Error>
            where
                Self: $crate::DBRelation<T>,
            {
                <Self as $crate::DBRelationFetch<T, $u_type>>::get_related_entity_or_fetch_with_conn(
                    &self, conn, client
                )
                .await
            }

            /// Get the related entities based on the relation type given in the type parameter
            pub async fn get_related_entity_or_fetch_with_pool<T>(
                &self,
                client: &$crate::DBClient,
            ) -> Result<Vec<<Self as $crate::DBRelation<T>>::ReturnedType>, crate::Error>
            where
                Self: $crate::DBRelation<T>,
            {
                <Self as $crate::DBRelationFetch<T, $u_type>>::get_related_entity_or_fetch_with_pool(
                    &self, client
                )
                .await
            }

            /// Get the related entities based on the relation type given in the type parameter
            pub async fn get_related_entity_or_fetch_as_task<T>(
                &self,
                client: &std::sync::Arc<$crate::DBClient>,
            ) -> Result<Vec<<Self as $crate::DBRelation<T>>::ReturnedType>, crate::Error>
            where
                Self: $crate::DBRelation<T>,
            {
                <Self as $crate::DBRelationFetch<T, $u_type>>::get_related_entity_or_fetch_as_task(
                    &self, client
                )
                .await
            }
        }
    };
}
pub(crate) use impl_db_relation_fetch_methods;
