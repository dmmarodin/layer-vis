extern crate console_error_panic_hook;
use cfg_if::cfg_if;

pub fn prep_logging() {
    console_error_panic_hook::set_once();

    cfg_if! {
        if #[cfg(feature = "console_log")] {
            use console_log;
            use log::{Level};
            console_log::init_with_level(Level::Debug).expect("Error initializing loggging");
        }
    }
}