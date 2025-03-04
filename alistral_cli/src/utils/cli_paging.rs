use inquire::{InquireError, Select};

#[derive(Debug)]
pub struct CLIPager {
    count: i32,
    max_count: i32,
}

impl CLIPager {
    pub fn new(max_count: i32) -> Self {
        Self {
            count: 0,
            max_count,
        }
    }

    pub fn execute<F>(&mut self, f: F) -> bool
    where
        F: Fn(),
    {
        f();

        self.inc()
    }

    pub fn inc(&mut self) -> bool {
        self.count += 1;

        if self.count == self.max_count {
            if Self::ask_continue() {
                self.count = 0;
            } else {
                return false;
            }
        }

        true
    }

    fn ask_continue() -> bool {
        loop {
            let options = vec!["Next", "Exit"];

            let ans: Result<&str, InquireError> = Select::new("", options).prompt();

            match ans {
                Ok(choice) => return choice == "Next",
                _ => println!("There was an error, please try again"),
            }
        }
    }
}
