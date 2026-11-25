//! Wall-clock budget shared by symbolic execution, simplification, and folding.

use std::time::{Duration, Instant};

/// A deadline measured from the moment it is created.
#[derive(Clone, Copy, Debug)]
pub struct Deadline {
    start: Instant,
    limit: Duration,
}

impl Deadline {
    /// Deadline `secs` seconds from now. Zero is already expired.
    pub fn after(secs: u64) -> Self {
        Self {
            start: Instant::now(),
            limit: Duration::from_secs(secs),
        }
    }

    pub fn expired(&self) -> bool {
        self.start.elapsed() >= self.limit
    }
}
