use serde::Deserialize;
use serde::Serialize;

pub mod last_listens;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ListenAction {
    Add,
    Remove,
}
