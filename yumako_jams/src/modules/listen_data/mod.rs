use serde::Deserialize;
use serde::Serialize;

pub mod clear_listens;
pub mod last_listens;
pub mod listen_interval;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ListenAction {
    Add,
    Remove,
    Replace,
}
