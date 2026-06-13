use serde::Serialize;
use std::path::PathBuf;

// Telemetry/diagnostics disabled. append_diagnostic_log is a no-op so that
// existing call sites compile without recording or transmitting any data.
pub fn append_diagnostic_log(_event: &str, _detail: impl Serialize) -> std::io::Result<()> {
    Ok(())
}

pub fn diagnostic_log_path() -> PathBuf {
    crate::paths::default_diagnostic_log_path()
}

#[doc(hidden)]
pub fn set_diagnostic_log_path_for_tests(_path: Option<PathBuf>) {}
