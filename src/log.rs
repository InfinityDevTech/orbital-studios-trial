use chrono::Local;
use colored::Colorize;

use crate::LOG_LEVEL;

pub fn log_debug(msg: &str) {
    if unsafe { LOG_LEVEL >= 4 } {
        let date = Local::now().format("%F %T").to_string();
        println!("[{}] - {}", date, msg.blue());
    }
}

pub fn log_info(msg: &str) {
    if unsafe { LOG_LEVEL >= 3 } {
        let date = Local::now().format("%F %T").to_string();
        println!("[{}] - {}", date, msg.green());
    }
}

pub fn log_warning(msg: &str) {
    if unsafe { LOG_LEVEL >= 2 } {
        let date = Local::now().format("%F %T").to_string();
        println!("[{}] - {}", date, msg.yellow());
    }
}

pub fn log_error(msg: &str) {
    if unsafe { LOG_LEVEL >= 1 } {
        let date = Local::now().format("%F %T").to_string();
        println!("[{}] - {}", date, msg.red());
    }
}
