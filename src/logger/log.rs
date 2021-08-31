use std::fmt::Display;

use slog::{crit, debug, error, info, trace, warn};
pub struct MyLogger {
    logger: slog::Logger,
}

impl MyLogger {
    pub fn new(logger: slog::Logger) -> MyLogger {
        MyLogger { logger }
    }

    pub fn trace<T: Display>(&self, msg: T) {
        trace!(self.logger, "{}", msg)
    }

    pub fn debug<T: Display>(&self, msg: T) {
        debug!(self.logger, "{}", msg)
    }

    pub fn info<T: Display>(&self, msg: T) {
        info!(self.logger, "{}", msg)
    }

    pub fn warn<T: Display>(&self, msg: T) {
        warn!(self.logger, "{}", msg)
    }

    pub fn error<T: Display>(&self, msg: T) {
        error!(self.logger, "{}", msg)
    }

    pub fn crit<T: Display>(&self, msg: T) {
        crit!(self.logger, "{}", msg)
    }
}
