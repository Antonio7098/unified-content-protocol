//! Error types for UCM operations.

use thiserror::Error;

/// Result type alias using UCM Error
pub type Result<T> = std::result::Result<T, Error>;

/// Error codes for categorization and i18n
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // Reference errors (E001-E099)
    E001BlockNotFound,
    E002InvalidBlockId,
    E003InvalidDocumentId,

    // Syntax errors (E100-E199)
    E100MalformedCommand,
    E101InvalidPath,
    E102InvalidValue,
    E103UnexpectedToken,

    // Validation errors (E200-E299)
    E200SchemaViolation,
    E201CycleDetected,
    E202InvalidStructure,
    E203OrphanedBlock,
    E204DuplicateId,

    // Concurrency errors (E300-E399)
    E300VersionConflict,
    E301TransactionTimeout,
    E302DeadlockDetected,
    E303TransactionNotFound,

    // Resource errors (E400-E499)
    E400DocumentSizeExceeded,
    E401MemoryLimitExceeded,
    E402BlockSizeExceeded,
    E403NestingDepthExceeded,
    E404EdgeCountExceeded,
    E405ExecutionTimeout,

    // Security errors (E500-E599)
    E500PathTraversal,
    E501DisallowedScheme,
    E502InvalidInput,

    // Internal errors (E900-E999)
    E900InternalError,
    E901SerializationError,
    E902IoError,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl ErrorCode {
    /// Get the string code (e.g., "E001")
    pub fn code(&self) -> &'static str {
        match self {
            Self::E001BlockNotFound => "E001",
            Self::E002InvalidBlockId => "E002",
            Self::E003InvalidDocumentId => "E003",
            Self::E100MalformedCommand => "E100",
            Self::E101InvalidPath => "E101",
            Self::E102InvalidValue => "E102",
            Self::E103UnexpectedToken => "E103",
            Self::E200SchemaViolation => "E200",
            Self::E201CycleDetected => "E201",
            Self::E202InvalidStructure => "E202",
            Self::E203OrphanedBlock => "E203",
            Self::E204DuplicateId => "E204",
            Self::E300VersionConflict => "E300",
            Self::E301TransactionTimeout => "E301",
            Self::E302DeadlockDetected => "E302",
            Self::E303TransactionNotFound => "E303",
            Self::E400DocumentSizeExceeded => "E400",
            Self::E401MemoryLimitExceeded => "E401",
            Self::E402BlockSizeExceeded => "E402",
            Self::E403NestingDepthExceeded => "E403",
            Self::E404EdgeCountExceeded => "E404",
            Self::E405ExecutionTimeout => "E405",
            Self::E500PathTraversal => "E500",
            Self::E501DisallowedScheme => "E501",
            Self::E502InvalidInput => "E502",
            Self::E900InternalError => "E900",
            Self::E901SerializationError => "E901",
            Self::E902IoError => "E902",
        }
    }

    /// Get a human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            Self::E001BlockNotFound => "Block does not exist",
            Self::E002InvalidBlockId => "Invalid block ID format",
            Self::E003InvalidDocumentId => "Invalid document ID format",
            Self::E100MalformedCommand => "Malformed UCL command",
            Self::E101InvalidPath => "Invalid path expression",
            Self::E102InvalidValue => "Invalid value",
            Self::E103UnexpectedToken => "Unexpected token",
            Self::E200SchemaViolation => "Content schema violation",
            Self::E201CycleDetected => "Cycle detected in structure",
            Self::E202InvalidStructure => "Invalid document structure",
            Self::E203OrphanedBlock => "Orphaned block detected",
            Self::E204DuplicateId => "Duplicate block ID",
            Self::E300VersionConflict => "Version conflict",
            Self::E301TransactionTimeout => "Transaction timeout",
            Self::E302DeadlockDetected => "Deadlock detected",
            Self::E303TransactionNotFound => "Transaction not found",
            Self::E400DocumentSizeExceeded => "Document size limit exceeded",
            Self::E401MemoryLimitExceeded => "Memory limit exceeded",
            Self::E402BlockSizeExceeded => "Block size limit exceeded",
            Self::E403NestingDepthExceeded => "Nesting depth limit exceeded",
            Self::E404EdgeCountExceeded => "Edge count limit exceeded",
            Self::E405ExecutionTimeout => "Execution timeout",
            Self::E500PathTraversal => "Path traversal attempt blocked",
            Self::E501DisallowedScheme => "Disallowed URL scheme",
            Self::E502InvalidInput => "Invalid input",
            Self::E900InternalError => "Internal error",
            Self::E901SerializationError => "Serialization error",
            Self::E902IoError => "I/O error",
        }
    }
}

/// Location in source for error reporting
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub length: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            line,
            column,
            offset: 0,
            length: 0,
        }
    }

    pub fn with_offset(mut self, offset: usize, length: usize) -> Self {
        self.offset = offset;
        self.length = length;
        self
    }
}

/// Main error type for UCM operations
#[derive(Debug, Error)]
pub enum Error {
    #[error("[{code}] {message}")]
    Ucm {
        code: ErrorCode,
        message: String,
        location: Option<Location>,
        context: Option<String>,
        suggestion: Option<String>,
    },

    #[error("Block not found: {0}")]
    BlockNotFound(String),

    #[error("Invalid block ID: {0}")]
    InvalidBlockId(String),

    #[error("Invalid document ID: {0}")]
    InvalidDocumentId(String),

    #[error("Cycle detected at block: {0}")]
    CycleDetected(String),

    #[error("Version conflict: expected {expected}, found {actual}")]
    VersionConflict { expected: u64, actual: u64 },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Parse error at line {line}, column {column}: {message}")]
    Parse {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Resource limit exceeded: {0}")]
    ResourceLimit(String),

    #[error("Security violation: {0}")]
    Security(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl Error {
    /// Create a new UCM error with full details
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self::Ucm {
            code,
            message: message.into(),
            location: None,
            context: None,
            suggestion: None,
        }
    }

    /// Add location information
    pub fn with_location(mut self, location: Location) -> Self {
        if let Self::Ucm {
            location: ref mut loc,
            ..
        } = self
        {
            *loc = Some(location);
        }
        self
    }

    /// Add context (e.g., the problematic command)
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        if let Self::Ucm {
            context: ref mut ctx,
            ..
        } = self
        {
            *ctx = Some(context.into());
        }
        self
    }

    /// Add a suggestion for fixing the error
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        if let Self::Ucm {
            suggestion: ref mut sug,
            ..
        } = self
        {
            *sug = Some(suggestion.into());
        }
        self
    }

    /// Get the error code if available
    pub fn code(&self) -> Option<ErrorCode> {
        match self {
            Self::Ucm { code, .. } => Some(*code),
            Self::BlockNotFound(_) => Some(ErrorCode::E001BlockNotFound),
            Self::InvalidBlockId(_) => Some(ErrorCode::E002InvalidBlockId),
            Self::InvalidDocumentId(_) => Some(ErrorCode::E003InvalidDocumentId),
            Self::CycleDetected(_) => Some(ErrorCode::E201CycleDetected),
            Self::VersionConflict { .. } => Some(ErrorCode::E300VersionConflict),
            Self::Validation(_) => Some(ErrorCode::E200SchemaViolation),
            Self::Parse { .. } => Some(ErrorCode::E100MalformedCommand),
            Self::ResourceLimit(_) => Some(ErrorCode::E400DocumentSizeExceeded),
            Self::Security(_) => Some(ErrorCode::E500PathTraversal),
            Self::Serialization(_) => Some(ErrorCode::E901SerializationError),
            Self::Io(_) => Some(ErrorCode::E902IoError),
            Self::Internal(_) => Some(ErrorCode::E900InternalError),
        }
    }
}

/// Validation issue (warning or info level)
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    pub code: ErrorCode,
    pub message: String,
    pub location: Option<Location>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

impl ValidationIssue {
    pub fn error(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            severity: ValidationSeverity::Error,
            code,
            message: message.into(),
            location: None,
            suggestion: None,
        }
    }

    pub fn warning(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            severity: ValidationSeverity::Warning,
            code,
            message: message.into(),
            location: None,
            suggestion: None,
        }
    }

    pub fn info(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            severity: ValidationSeverity::Info,
            code,
            message: message.into(),
            location: None,
            suggestion: None,
        }
    }

    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_strings() {
        assert_eq!(ErrorCode::E001BlockNotFound.code(), "E001");
        assert_eq!(
            ErrorCode::E001BlockNotFound.description(),
            "Block does not exist"
        );
    }

    #[test]
    fn test_error_with_details() {
        let err = Error::new(ErrorCode::E001BlockNotFound, "Block 'blk_abc' not found")
            .with_location(Location::new(10, 5))
            .with_context("MOVE blk_abc TO blk_root")
            .with_suggestion("Did you mean 'blk_abd'?");

        assert_eq!(err.code(), Some(ErrorCode::E001BlockNotFound));
    }
}
