// SPDX-License-Identifier: Apache-2.0

#![doc = include_str!("../README.md")]

pub mod diag;
pub mod error;
pub mod quiet;

use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

/// A trait that defines the interface of a logger.
pub trait Logger {
    /// Logs an trace message (only with debug enabled).
    fn trace(&self, message: &str);

    /// Logs an info message.
    fn info(&self, message: &str);

    /// Logs a warning message.
    fn warn(&self, message: &str);

    /// Logs an error message.
    fn error(&self, message: &str);

    /// Logs a success message.
    fn success(&self, message: &str);

    /// Logs a newline.
    fn newline(&self, count: usize);

    /// Indents the logger.
    fn indent(&self, count: usize);

    /// Stops a loading message.
    fn done(&self);

    /// Adds a style to the logger.
    fn add_style(&self, name: &str, styles: Vec<&'static str>) -> &Self;

    /// Logs a loading message with a spinner.
    fn loading(&self, message: &str);

    /// Forces the logger to not print a newline for the next message.
    fn same(&self) -> &Self;

    /// Logs a message without icon.
    fn log(&self, message: &str);
}

/// A generic logger that can be used to log messages to the console.
/// This logger is thread-safe and can be cloned.
#[derive(Default, Clone)]
pub struct ConsoleLogger {
    logger: Arc<Mutex<paris::Logger<'static>>>,
    debug_level: u8,
}

impl ConsoleLogger {
    /// Creates a new logger.
    #[must_use]
    pub fn new(debug_level: u8) -> Self {
        ConsoleLogger {
            logger: Arc::new(Mutex::new(paris::Logger::new())),
            debug_level,
        }
    }
}

impl Logger for ConsoleLogger {
    /// Logs an trace message (only with debug enabled).
    fn trace(&self, message: &str) {
        if self.debug_level > 0 {
            _ = self
                .logger
                .lock()
                .expect("Failed to lock logger")
                .log(message);
        }
    }

    /// Logs an info message.
    fn info(&self, message: &str) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .info(message);
    }

    /// Logs a warning message.
    fn warn(&self, message: &str) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .warn(message);
    }

    /// Logs an error message.
    fn error(&self, message: &str) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .error(message);
    }

    /// Logs a success message.
    fn success(&self, message: &str) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .success(message);
    }

    /// Logs a newline.
    fn newline(&self, count: usize) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .newline(count);
    }

    /// Indents the logger.
    fn indent(&self, count: usize) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .indent(count);
    }

    /// Stops a loading message.
    fn done(&self) {
        _ = self.logger.lock().expect("Failed to lock logger").done();
    }

    /// Adds a style to the logger.
    fn add_style(&self, name: &str, styles: Vec<&'static str>) -> &Self {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .add_style(name, styles);
        self
    }

    /// Logs a loading message with a spinner.
    fn loading(&self, message: &str) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .loading(message);
    }

    /// Forces the logger to not print a newline for the next message.
    fn same(&self) -> &Self {
        _ = self.logger.lock().expect("Failed to lock logger").same();
        self
    }

    /// Logs a message without icon.
    fn log(&self, message: &str) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .log(message);
    }
}

/// A logger that does not log anything.
#[derive(Default, Clone)]
pub struct NullLogger {}

impl NullLogger {
    /// Creates a new logger.
    #[must_use]
    pub fn new() -> Self {
        NullLogger {}
    }
}

impl Logger for NullLogger {
    /// Logs an trace message (only with debug enabled).
    fn trace(&self, _: &str) {}

    /// Logs an info message.
    fn info(&self, _: &str) {}

    /// Logs a warning message.
    fn warn(&self, _: &str) {}

    /// Logs an error message.
    fn error(&self, _: &str) {}

    /// Logs a success message.
    fn success(&self, _: &str) {}

    /// Logs a newline.
    fn newline(&self, _: usize) {}

    /// Indents the logger.
    fn indent(&self, _: usize) {}

    /// Stops a loading message.
    fn done(&self) {}

    /// Adds a style to the logger.
    fn add_style(&self, _: &str, _: Vec<&'static str>) -> &Self {
        self
    }

    /// Logs a loading message with a spinner.
    fn loading(&self, _: &str) {}

    /// Forces the logger to not print a newline for the next message.
    fn same(&self) -> &Self {
        self
    }

    /// Logs a message without icon.
    fn log(&self, _: &str) {}
}

/// A logger that can be used in unit or integration tests.
/// This logger is thread-safe and can be cloned.
#[derive(Default, Clone)]
pub struct TestLogger {
    logger: Arc<Mutex<paris::Logger<'static>>>,
    warn_count: Arc<AtomicUsize>,
    error_count: Arc<AtomicUsize>,
}

impl TestLogger {
    /// Creates a new logger.
    #[must_use]
    pub fn new() -> Self {
        TestLogger {
            logger: Arc::new(Mutex::new(paris::Logger::new())),
            warn_count: Arc::new(AtomicUsize::new(0)),
            error_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Returns the number of warning messages logged.
    #[must_use]
    pub fn warn_count(&self) -> usize {
        self.warn_count.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Returns the number of error messages logged.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.error_count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Logger for TestLogger {
    /// Logs a trace message (only with debug enabled).
    fn trace(&self, _message: &str) {}

    /// Logs an info message.
    fn info(&self, message: &str) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .info(message);
    }

    /// Logs a warning message.
    fn warn(&self, message: &str) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .warn(message);
        _ = self
            .warn_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Logs an error message.
    fn error(&self, message: &str) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .error(message);
        _ = self
            .error_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Logs a success message.
    fn success(&self, message: &str) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .success(message);
    }

    /// Logs a newline.
    fn newline(&self, count: usize) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .newline(count);
    }

    /// Indents the logger.
    fn indent(&self, count: usize) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .indent(count);
    }

    /// Stops a loading message.
    fn done(&self) {
        _ = self.logger.lock().expect("Failed to lock logger").done();
    }

    /// Adds a style to the logger.
    fn add_style(&self, name: &str, styles: Vec<&'static str>) -> &Self {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .add_style(name, styles);
        self
    }

    /// Logs a loading message with a spinner.
    fn loading(&self, message: &str) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .loading(message);
    }

    /// Forces the logger to not print a newline for the next message.
    fn same(&self) -> &Self {
        _ = self.logger.lock().expect("Failed to lock logger").same();
        self
    }

    /// Logs a message without icon.
    fn log(&self, message: &str) {
        _ = self
            .logger
            .lock()
            .expect("Failed to lock logger")
            .log(message);
    }
}

/// TODO
#[derive(Debug, PartialEq)]
#[must_use]
pub struct DiagnosticResult<E> {
    /// Exposed for macro.
    pub is_fatal: bool,
    /// Exposed for macro.
    pub messages: Vec<E>,
}

/// TODO - DOn't use vec and optimise this.
pub fn combine<E>(lhs: Vec<E>, rhs: Vec<E>) -> Vec<E> {
    let mut result = Vec::with_capacity(lhs.len() + rhs.len());
    result.extend(lhs);
    result.extend(rhs);
    result
}

/// TODO
pub fn tell<E>(msg: E) -> DiagnosticResult<E> {
    DiagnosticResult {
        is_fatal: false,
        messages: vec![msg],
    }
}

impl<E> DiagnosticResult<E> {
    /// TODO
    pub fn new() -> DiagnosticResult<E> {
        DiagnosticResult {
            is_fatal: false,
            messages: Vec::new(),
        }
    }

    /// Clears all messages.
    pub fn clear(&mut self) {
        self.is_fatal = false;
        self.messages.clear();
    }

    /// TODO
    pub fn fail(self) -> DiagnosticResult<E> {
        match self {
            DiagnosticResult { messages, .. } => DiagnosticResult {
                is_fatal: true,
                messages,
            },
        }
    }
    /// TODO
    pub fn tell(&mut self, msg: E) {
        self.messages.push(msg);
    }

    /// TODO
    pub fn merge(&mut self, other: DiagnosticResult<E>) {
        self.messages.extend(other.messages)
    }

    /// TODO
    pub fn has_messages(&self) -> bool {
        match self {
            DiagnosticResult { messages, .. } => !messages.is_empty(),
        }
    }
    /// TODO
    pub fn and_then<F>(self, f: F) -> DiagnosticResult<E>
    where
        F: FnOnce() -> DiagnosticResult<E>,
    {
        match self {
            DiagnosticResult {
                is_fatal: false,
                messages: lhs,
            } => match f() {
                DiagnosticResult {
                    is_fatal,
                    messages: rhs,
                } => DiagnosticResult {
                    is_fatal,
                    messages: combine(lhs, rhs),
                },
            },
            DiagnosticResult {
                is_fatal: true,
                messages,
            } => DiagnosticResult {
                is_fatal: true,
                messages,
            },
        }
    }

    /// TODO
    pub fn is_ok(&self) -> bool {
        !self.is_fatal
    }
    /// TODO
    pub fn is_err(&self) -> bool {
        self.is_fatal
    }

    /// TODO
    pub fn messages(self) -> Vec<E> {
        self.messages
    }
}

/// TODO - document
#[macro_export]
macro_rules! collect_diagnostics {
    ($result:expr) => {
        $result
    };
    ($comp:expr; $($tail:tt)*) => {
        match $comp {
            $crate::DiagnosticResult { is_fatal, messages } => {
                if is_fatal {
                    $crate::DiagnosticResult { is_fatal, messages }
                } else {
                    match collect_diagnostics!($($tail)*) {
                        $crate::DiagnosticResult { is_fatal, messages: rhs } =>
                          $crate::DiagnosticResult { is_fatal, messages: $crate::combine(messages,rhs) }
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DiagnosticResult;

    #[test]
    fn check_macro() {
        fn can_pass_one() -> DiagnosticResult<String> {
            let mut result = DiagnosticResult::new();
            result.tell("Test".into());
            result
        }
        fn cant_pass_one() -> DiagnosticResult<String> {
            let mut result = DiagnosticResult::new();
            result.tell("Fail Test".into());
            result.fail()
        }
        fn can_pass_two() -> DiagnosticResult<String> {
            let mut result = DiagnosticResult::new();
            result.tell("Test2.1".into());
            result.tell("Test2.2".into());
            result
        }

        let result: DiagnosticResult<String> = collect_diagnostics! {
          can_pass_one();
          can_pass_two()
        };
        assert_eq!(
            result,
            DiagnosticResult {
                is_fatal: false,
                messages: vec!("Test".into(), "Test2.1".into(), "Test2.2".into()),
            }
        );
        let result: DiagnosticResult<String> = collect_diagnostics! {
          can_pass_one();
          cant_pass_one();
          can_pass_two()
        };
        assert_eq!(
            result,
            DiagnosticResult {
                is_fatal: true,
                messages: vec!("Test".into(), "Fail Test".into()),
            }
        );
    }
}
