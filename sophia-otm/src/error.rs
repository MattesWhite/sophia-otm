//! Handling of errors.

/// Error raised when a triple is missing to construct an instance.
#[derive(Debug, Clone, thiserror::Error)]
#[error("RDF graph misses triple: {}", 0)]
pub struct MissingTriple(pub String);
