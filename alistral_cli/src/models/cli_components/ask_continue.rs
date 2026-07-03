use inquire::Select;

pub fn cli_ask_continue() -> bool {
    let options = vec!["Next", "Exit"];

    Select::new("", options).prompt().unwrap() == "Next"
}

