use std::env;

pub fn in_offline_mode() -> bool {
    for (key, value) in env::vars() {
        if &key == "OFFLINE" && value == "true" {
            return true;
        }
    }

    false
}
