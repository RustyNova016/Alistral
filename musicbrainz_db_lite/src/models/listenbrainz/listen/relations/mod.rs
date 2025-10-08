use crate::DBRelation;
use crate::User;

pub mod listen_recordings;
pub mod messybrainz_submission;
pub mod recording;

pub struct UserListenDBRel;

impl DBRelation<UserListenDBRel> for super::Listen {
    type ReturnedType = User;

    fn get_join_statement() -> &'static str {
        "INNER JOIN `users` ON `listens`.`user` = `users`.`name`"
    }
}
