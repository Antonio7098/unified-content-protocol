//! Performance and execution configuration for the UCM engine.
//!
//! This module provides configuration options for controlling engine
//! performance, resource limits, and execution behavior.

use serde::{Deserialize, Serialize};

/// Performance configuration for the engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Maximum memory usage in MB (advisory)
    pub max_memory_mb: usize,
    /// Maximum operations per batch
    pub max_operations_per_batch: usize,
    /// Cache size in MB
    pub cache_size_mb: usize,
    /// Enable streaming for large operations
    pub enable_streaming: bool,
    /// Maximum concurrent operations (for async)
    pub max_concurrent_ops: usize,
    /// Operation timeout in milliseconds
    pub operation_timeout_ms: u64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,
            max_operations_per_batch: 1000,
            cache_size_mb: 64,
            enable_streaming: true,
            max_concurrent_ops: 4,
            operation_timeout_ms: 30000,
        }
    }
}

impl PerformanceConfig {
    /// Create a minimal configuration for resource-constrained environments
    pub fn minimal() -> Self {
        Self {
            max_memory_mb: 64,
            max_operations_per_batch: 100,
            cache_size_mb: 8,
            enable_streaming: false,
            max_concurrent_ops: 1,
            operation_timeout_ms: 10000,
        }
    }

    /// Create a high-performance configuration
    pub fn high_performance() -> Self {
        Self {
            max_memory_mb: 2048,
            max_operations_per_batch: 10000,
            cache_size_mb: 256,
            enable_streaming: true,
            max_concurrent_ops: 16,
            operation_timeout_ms: 60000,
        }
    }
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Allowed URL schemes for media sources
    pub allowed_schemes: Vec<String>,
    /// Maximum URL length
    pub max_url_length: usize,
    /// Block external resource loading
    pub block_external_resources: bool,
    /// Sanitize content on input
    pub sanitize_content: bool,
    /// Maximum content size per block in bytes
    pub max_block_size: usize,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            allowed_schemes: vec!["http".to_string(), "https".to_string(), "data".to_string()],
            max_url_length: 2048,
            block_external_resources: false,
            sanitize_content: true,
            max_block_size: 1024 * 1024, // 1MB
        }
    }
}

impl SecurityConfig {
    /// Create a strict security configuration
    pub fn strict() -> Self {
        Self {
            allowed_schemes: vec!["https".to_string()],
            max_url_length: 1024,
            block_external_resources: true,
            sanitize_content: true,
            max_block_size: 256 * 1024, // 256KB
        }
    }

    /// Validate a URL against security settings
    pub fn validate_url(&self, url: &str) -> Result<(), String> {
        if url.len() > self.max_url_length {
            return Err(format!(
                "URL exceeds maximum length of {} characters",
                self.max_url_length
            ));
        }

        // Check scheme
        let scheme = url.split(':').next().unwrap_or("");
        if !self.allowed_schemes.contains(&scheme.to_lowercase()) {
            return Err(format!(
                "URL scheme '{}' is not allowed. Allowed schemes: {:?}",
                scheme, self.allowed_schemes
            ));
        }

        // Check for path traversal attempts
        if url.contains("..") {
            return Err("Path traversal attempts are not allowed".to_string());
        }

        Ok(())
    }
}

/// Combined engine configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EngineConfig {
    pub performance: PerformanceConfig,
    pub security: SecurityConfig,
}

impl EngineConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_performance(mut self, config: PerformanceConfig) -> Self {
        self.performance = config;
        self
    }

    pub fn with_security(mut self, config: SecurityConfig) -> Self {
        self.security = config;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = EngineConfig::default();
        assert_eq!(config.performance.max_memory_mb, 512);
        assert!(config
            .security
            .allowed_schemes
            .contains(&"https".to_string()));
    }

    #[test]
    fn test_minimal_config() {
        let config = PerformanceConfig::minimal();
        assert!(config.max_memory_mb < 100);
        assert!(!config.enable_streaming);
    }

    #[test]
    fn test_high_performance_config() {
        let config = PerformanceConfig::high_performance();
        assert!(config.max_memory_mb > 1000);
        assert!(config.max_concurrent_ops > 8);
    }

    #[test]
    fn test_strict_security() {
        let config = SecurityConfig::strict();
        assert!(config.block_external_resources);
        assert_eq!(config.allowed_schemes.len(), 1);
    }

    #[test]
    fn test_url_validation() {
        let config = SecurityConfig::default();

        // Valid URLs
        assert!(config.validate_url("https://example.com/image.jpg").is_ok());
        assert!(config.validate_url("http://example.com/file").is_ok());
        assert!(config.validate_url("data:image/png;base64,abc123").is_ok());

        // Invalid scheme
        assert!(config.validate_url("ftp://example.com/file").is_err());

        // Path traversal
        assert!(config
            .validate_url("https://example.com/../etc/passwd")
            .is_err());
    }

    #[test]
    fn test_url_length_limit() {
        let config = SecurityConfig {
            max_url_length: 50,
            ..Default::default()
        };

        let short_url = "https://example.com/a";
        let long_url = "https://example.com/".to_string() + &"a".repeat(100);

        assert!(config.validate_url(short_url).is_ok());
        assert!(config.validate_url(&long_url).is_err());
    }
}
