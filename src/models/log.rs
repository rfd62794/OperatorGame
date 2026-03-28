// Moved from models.rs
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Log persistence types (Sprint F.1b Task F.1)
// ---------------------------------------------------------------------------

/// Outcome category for a persisted log entry — used for colour-coding in the
/// Logs tab without resorting to string-search heuristics.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogOutcome {
    Victory,
    Failure,
    CritFail,
    System,
}

/// A persisted log line stored in `GameState.combat_log`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Unix timestamp (seconds) when the entry was created.
    pub timestamp: u64,
    /// Formatted human-readable message produced by `log_engine`.
    pub message: String,
    /// Category used for colour selection in the Logs tab.
    pub outcome: LogOutcome,
}
