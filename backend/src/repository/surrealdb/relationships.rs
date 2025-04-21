//! Utilities for working with relationship formats in SurrealDB

use log::debug;

/// Relationship type for one-to-one ownership
pub const OWNS_REL: &str = "owns";

/// Relationship type for parent-child (one-to-many) relationships
pub const HAS_CHILD_REL: &str = "has_child";

/// Relationship type for many-to-many associations
pub const ASSOCIATES_WITH_REL: &str = "associates_with";

/// Formats a relationship name for ownership relationships
pub fn owns_rel_name(entity_name: &str) -> String {
    let rel_name = format!("{}_{}", OWNS_REL, entity_name);
    debug!("Generated ownership relationship name: {}", rel_name);
    rel_name
}

/// Formats a relationship name for parent-child relationships
pub fn has_child_rel_name(entity_name: &str) -> String {
    let rel_name = format!("{}_{}", HAS_CHILD_REL, entity_name);
    debug!("Generated parent-child relationship name: {}", rel_name);
    rel_name
}

/// Formats a relationship name for association relationships
pub fn associates_with_rel_name(entity_name: &str) -> String {
    let rel_name = format!("{}_{}", ASSOCIATES_WITH_REL, entity_name);
    debug!("Generated association relationship name: {}", rel_name);
    rel_name
}
