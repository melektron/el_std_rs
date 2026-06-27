/*
ELEKTRON © 2026 - now
Written by melektron
www.elektron.work
08.03.26, 15:25
All rights reserved.

This source code is licensed under the Apache-2.0 license found in the
LICENSE file in the root directory of this source tree.
*/

//! Adds methods to log a message if a result is an error.

use log::{debug, error, info, trace, warn};

/// Adds methods to print a logging message for any error case
pub trait OrLog {
    /// Logs the provided `msg` with error level if Result is `Err(...)`
    fn or_error(&self, msg: &str);
    /// Logs the provided `msg` with warn level if Result is `Err(...)`
    fn or_warn(&self, msg: &str);
    /// Logs the provided `msg` with info level if Result is `Err(...)`
    fn or_info(&self, msg: &str);
    /// Logs the provided `msg` with debug level if Result is `Err(...)`
    fn or_debug(&self, msg: &str);
    /// Logs the provided `msg` with trace level if Result is `Err(...)`
    fn or_trace(&self, msg: &str);
}

impl<T, E> OrLog for Result<T, E> {
    fn or_error(&self, msg: &str) {
        if self.is_err() {
            error!("{msg}")
        }
    }

    fn or_warn(&self, msg: &str) {
        if self.is_err() {
            warn!("{msg}")
        }
    }

    fn or_info(&self, msg: &str) {
        if self.is_err() {
            info!("{msg}")
        }
    }

    fn or_debug(&self, msg: &str) {
        if self.is_err() {
            debug!("{msg}")
        }
    }

    fn or_trace(&self, msg: &str) {
        if self.is_err() {
            trace!("{msg}")
        }
    }
}

#[cfg(test)]
mod tests {

    use log::Level;

    use super::*;

    #[test]
    fn ok_or_error() {
        testing_logger::setup();
        Ok::<u8, &str>(1).or_error("test log");
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 0, "nothing should have been logged");
        });
    }
    #[test]
    fn ok_or_warn() {
        testing_logger::setup();
        Ok::<u8, &str>(1).or_warn("test log");
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 0, "nothing should have been logged");
        });
    }
    #[test]
    fn ok_or_info() {
        testing_logger::setup();
        Ok::<u8, &str>(1).or_info("test log");
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 0, "nothing should have been logged");
        });
    }
    #[test]
    fn ok_or_debug() {
        testing_logger::setup();
        Ok::<u8, &str>(1).or_debug("test log");
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 0, "nothing should have been logged");
        });
    }
    #[test]
    fn ok_or_trace() {
        testing_logger::setup();
        Ok::<u8, &str>(1).or_trace("test log");
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 0, "nothing should have been logged");
        });
    }

    #[test]
    fn err_or_error() {
        testing_logger::setup();
        Err::<u8, &str>("hi").or_error("test log");
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1, "nothing should have been logged");
            assert_eq!(captured_logs[0].body, "test log", "invalid log body");
            assert_eq!(captured_logs[0].level, Level::Error, "invalid log level");
        });
    }
    #[test]
    fn err_or_warn() {
        testing_logger::setup();
        Err::<u8, &str>("hi").or_warn("test log");
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1, "nothing should have been logged");
            assert_eq!(captured_logs[0].body, "test log", "invalid log body");
            assert_eq!(captured_logs[0].level, Level::Warn, "invalid log level");
        });
    }
    #[test]
    fn err_or_info() {
        testing_logger::setup();
        Err::<u8, &str>("hi").or_info("test log");
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1, "nothing should have been logged");
            assert_eq!(captured_logs[0].body, "test log", "invalid log body");
            assert_eq!(captured_logs[0].level, Level::Info, "invalid log level");
        });
    }
    #[test]
    fn err_or_debug() {
        testing_logger::setup();
        Err::<u8, &str>("hi").or_debug("test log");
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1, "nothing should have been logged");
            assert_eq!(captured_logs[0].body, "test log", "invalid log body");
            assert_eq!(captured_logs[0].level, Level::Debug, "invalid log level");
        });
    }
    #[test]
    fn err_or_trace() {
        testing_logger::setup();
        Err::<u8, &str>("hi").or_trace("test log");
        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1, "nothing should have been logged");
            assert_eq!(captured_logs[0].body, "test log", "invalid log body");
            assert_eq!(captured_logs[0].level, Level::Trace, "invalid log level");
        });
    }
}
