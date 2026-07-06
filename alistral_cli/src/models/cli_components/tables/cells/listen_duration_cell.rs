use crate::datastructures::formaters::human_time::HumanTimePrinter;
use crate::models::datastructures::tops::printer::top_cell::TopCell;

pub struct ListenDurationCell(pub TopCell<HumanTimePrinter>);