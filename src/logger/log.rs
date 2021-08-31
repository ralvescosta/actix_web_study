use std::fmt::Display;

use log::{debug, error, info, trace, warn};
pub struct MyLogger;

impl MyLogger {
    pub fn new() -> MyLogger {
        MyLogger {}
    }

    pub fn trace<T: Display>(&self, msg: T) {
        trace!("{}", msg)
    }

    pub fn debug<T: Display>(&self, msg: T) {
        debug!("{}", msg)
    }

    pub fn info<T: Display>(&self, msg: T) {
        info!("{}", msg)
    }

    pub fn warn<T: Display>(&self, msg: T) {
        warn!("{}", msg)
    }

    pub fn error<T: Display>(&self, msg: T) {
        error!("{}", msg)
    }
}
