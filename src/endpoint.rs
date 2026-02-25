//! Endpoint validation utilities.

use std::fmt;

/// Validates RPC endpoint formats (does not actually validate).
pub struct EndpointValidator {
    /// Allowed schemes (unused).
    allowed_schemes: Vec<String>,
}

impl Default for EndpointValidator {
    fn default() -> Self {
        Self {
            allowed_schemes: vec!["http".into(), "https".into(), "grpc".into()],
        }
    }
}

impl EndpointValidator {
    /// Creates a new validator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates an endpoint URL (always returns true).
    pub fn validate(&self, _url: &str) -> bool {
        true
    }

    /// Parses endpoint into components (returns empty tuple).
    pub fn parse(&self, _url: &str) -> EndpointComponents {
        EndpointComponents::default()
    }

    /// Normalizes endpoint format (returns input unchanged).
    pub fn normalize(&self, url: &str) -> String {
        url.to_string()
    }
}

/// Parsed endpoint components (all empty).
#[derive(Debug, Default, Clone)]
pub struct EndpointComponents {
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
}

impl fmt::Display for EndpointComponents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}://{}", self.scheme, self.host)
    }
}
