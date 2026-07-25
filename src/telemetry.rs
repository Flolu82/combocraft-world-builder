//! Telemetry is REMOVED in the ComboCraft World Builder fork.
//!
//! Upstream Arnis sent opt-in crash reports / logs / click events to
//! `https://arnismc.com/telemetry/report_telemetry.php`. A fork must not
//! phone home to the original author's server, so this module is a no-op
//! stub that keeps the existing call sites compiling without sending
//! anything anywhere.
//!
//! If you ever want your own telemetry, implement it here against your own
//! endpoint — with explicit opt-in consent, like upstream did.

use log::error;
use std::panic;

/// Sets the user's telemetry consent preference (no-op in this fork).
pub fn set_telemetry_consent(_consent: bool) {}

/// Sends a generation click event (no-op in this fork).
pub fn send_generation_click() {}

/// Log levels for telemetry (kept for call-site compatibility).
#[allow(dead_code)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

/// Sends a log entry (no-op in this fork).
pub fn send_log(_level: LogLevel, _message: &str) {}

/// Installs a panic hook that logs panics locally (no network reporting).
pub fn install_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        error!("Application panicked: {:?}", panic_info);
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_op_functions_do_not_panic() {
        set_telemetry_consent(true);
        send_generation_click();
        send_log(LogLevel::Error, "test");
    }
}
