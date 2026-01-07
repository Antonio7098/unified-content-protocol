//! Version tracking for optimistic concurrency control.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Block version for optimistic concurrency control
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
    /// Monotonically increasing counter
    pub counter: u64,
    /// Timestamp of last modification
    pub timestamp: DateTime<Utc>,
}

impl Version {
    /// Create an initial version
    pub fn initial() -> Self {
        Self {
            counter: 1,
            timestamp: Utc::now(),
        }
    }

    /// Increment the version
    pub fn increment(&mut self) {
        self.counter += 1;
        self.timestamp = Utc::now();
    }

    /// Create the next version
    pub fn next(&self) -> Self {
        Self {
            counter: self.counter + 1,
            timestamp: Utc::now(),
        }
    }

    /// Check if this version is newer than another
    pub fn is_newer_than(&self, other: &Version) -> bool {
        self.counter > other.counter
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::initial()
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}", self.counter)
    }
}

/// Document version with additional metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentVersion {
    /// Version counter
    pub counter: u64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Hash of document state (for integrity checking)
    pub state_hash: [u8; 8],
}

impl DocumentVersion {
    /// Create initial document version
    pub fn initial() -> Self {
        Self {
            counter: 1,
            timestamp: Utc::now(),
            state_hash: [0u8; 8],
        }
    }

    /// Increment version with new state hash
    pub fn increment(&mut self, state_hash: [u8; 8]) {
        self.counter += 1;
        self.timestamp = Utc::now();
        self.state_hash = state_hash;
    }

    /// Check if versions match
    pub fn matches(&self, other: &DocumentVersion) -> bool {
        self.counter == other.counter && self.state_hash == other.state_hash
    }
}

impl Default for DocumentVersion {
    fn default() -> Self {
        Self::initial()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_increment() {
        let mut v = Version::initial();
        assert_eq!(v.counter, 1);

        v.increment();
        assert_eq!(v.counter, 2);
    }

    #[test]
    fn test_version_comparison() {
        let v1 = Version::initial();
        let v2 = v1.next();

        assert!(v2.is_newer_than(&v1));
        assert!(!v1.is_newer_than(&v2));
    }

    #[test]
    fn test_document_version() {
        let mut dv = DocumentVersion::initial();
        let hash = [1u8; 8];
        dv.increment(hash);

        assert_eq!(dv.counter, 2);
        assert_eq!(dv.state_hash, hash);
    }
}
