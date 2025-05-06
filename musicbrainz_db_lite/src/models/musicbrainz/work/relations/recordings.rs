use crate::DBRelation;
use crate::Recording;
use crate::Work;
use crate::models::shared_traits::db_relation::RecordingWorkDBRel;

impl DBRelation<RecordingWorkDBRel> for Work {
    type ReturnedType = Recording;

    fn get_join_statement() -> &'static str {
        "INNER JOIN l_recordings_works as rel ON rel.entity1 = works.id
        INNER JOIN recordings ON recordings.id = rel.entity0"
    }
}
