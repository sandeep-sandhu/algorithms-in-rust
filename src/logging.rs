// Utilities for logging

use chrono::{DateTime, Local};

use log::{Record, Level, Metadata};

pub struct SimpleInfoLogger;
pub struct SimpleDebugLogger;

impl log::Log for SimpleInfoLogger {

    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now: DateTime<Local> = Local::now();
            println!("{}:{}:{}", now.format("%Y-%m-%d %T"), record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

impl log::Log for SimpleDebugLogger {

    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now: DateTime<Local> = Local::now();
            println!("{}:{}:{}", now.format("%Y-%m-%d %T"), record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

