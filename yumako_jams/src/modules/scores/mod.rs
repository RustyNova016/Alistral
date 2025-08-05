pub mod rank;
pub mod bump;
pub mod overdue_count;
pub mod overdue_duration;
use serde::Deserialize;
use serde::Serialize;

pub mod listenrate;
pub mod sort;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ScoreMerging {
    Replace,
    Add,
    Sub,
    Multiply,
    Divide,
}
