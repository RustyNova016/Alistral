use serde::Deserialize;
use serde::Serialize;

//pub mod rank;
pub mod bump;
pub mod listenrate;
pub mod overdue_count;
pub mod overdue_duration;
pub mod sort;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ScoreMerging {
    Replace,
    Add,
    Sub,
    Multiply,
    Divide,
}
