//! Test helper utilities for unit and integration tests
//!
//! This module provides common utilities, mock data factories, and helper functions
//! used across different test modules in the application.

use crate::models::{Crate, NewCrate, NewRustacean, NewUser, Role, RoleCode, Rustacean, User};
use diesel::result::Error as DieselError;
use std::collections::HashMap;

/// Test data factory for creating mock users
pub struct UserFactory;

impl UserFactory {
    pub fn create_user() -> User {
        User {
            id: 1,
            username: "testuser".to_string(),
            password: "$argon2id$v=19$m=19456,t=2,p=1$VE0e2XSF+9Tz7/hYP8I9dQ$J4moa2MM3d6FRYiW6BHjbVa7SdKHHjhZXJ4pGSXnKl8".to_string(),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0).unwrap().naive_utc(),
        }
    }

    pub fn create_user_with_id(id: i32) -> User {
        User {
            id,
            username: format!("testuser{}", id),
            password: "$argon2id$v=19$m=19456,t=2,p=1$VE0e2XSF+9Tz7/hYP8I9dQ$J4moa2MM3d6FRYiW6BHjbVa7SdKHHjhZXJ4pGSXnKl8".to_string(),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0).unwrap().naive_utc(),
        }
    }

    pub fn create_user_with_username(username: &str) -> User {
        User {
            id: 1,
            username: username.to_string(),
            password: "$argon2id$v=19$m=19456,t=2,p=1$VE0e2XSF+9Tz7/hYP8I9dQ$J4moa2MM3d6FRYiW6BHjbVa7SdKHHjhZXJ4pGSXnKl8".to_string(),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0).unwrap().naive_utc(),
        }
    }

    pub fn create_new_user() -> NewUser {
        NewUser {
            username: "newuser".to_string(),
            password: "hashed_password".to_string(),
        }
    }

    pub fn create_new_user_with_username(username: &str) -> NewUser {
        NewUser {
            username: username.to_string(),
            password: "hashed_password".to_string(),
        }
    }
}

/// Test data factory for creating mock rustaceans
pub struct RustaceanFactory;

impl RustaceanFactory {
    pub fn create_rustacean() -> Rustacean {
        Rustacean {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    pub fn create_rustacean_with_id(id: i32) -> Rustacean {
        Rustacean {
            id,
            name: format!("User {}", id),
            email: format!("user{}@example.com", id),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    pub fn create_rustacean_with_email(email: &str) -> Rustacean {
        Rustacean {
            id: 1,
            name: "John Doe".to_string(),
            email: email.to_string(),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    pub fn create_new_rustacean() -> NewRustacean {
        NewRustacean {
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
        }
    }

    pub fn create_new_rustacean_with_data(name: &str, email: &str) -> NewRustacean {
        NewRustacean {
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    pub fn create_multiple_rustaceans(count: usize) -> Vec<Rustacean> {
        (1..=count)
            .map(|i| Self::create_rustacean_with_id(i as i32))
            .collect()
    }
}

/// Test data factory for creating mock crates
pub struct CrateFactory;

impl CrateFactory {
    pub fn create_crate() -> Crate {
        Crate {
            id: 1,
            rustacean_id: 1,
            code: "test_crate".to_string(),
            name: "Test Crate".to_string(),
            version: "0.1.0".to_string(),
            description: Some("A test crate".to_string()),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    pub fn create_crate_with_id(id: i32) -> Crate {
        Crate {
            id,
            rustacean_id: 1,
            code: format!("test_crate_{}", id),
            name: format!("Test Crate {}", id),
            version: "0.1.0".to_string(),
            description: Some(format!("Test crate number {}", id)),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    pub fn create_crate_for_rustacean(rustacean_id: i32) -> Crate {
        Crate {
            id: 1,
            rustacean_id,
            code: "test_crate".to_string(),
            name: "Test Crate".to_string(),
            version: "0.1.0".to_string(),
            description: Some("A test crate".to_string()),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    pub fn create_new_crate() -> NewCrate {
        NewCrate {
            rustacean_id: 1,
            code: "new_test_crate".to_string(),
            name: "New Test Crate".to_string(),
            version: "0.2.0".to_string(),
            description: Some("A new test crate".to_string()),
        }
    }

    pub fn create_new_crate_for_rustacean(rustacean_id: i32) -> NewCrate {
        NewCrate {
            rustacean_id,
            code: "new_test_crate".to_string(),
            name: "New Test Crate".to_string(),
            version: "0.2.0".to_string(),
            description: Some("A new test crate".to_string()),
        }
    }

    pub fn create_new_crate_with_data(
        rustacean_id: i32,
        code: &str,
        name: &str,
        version: &str,
        description: Option<String>,
    ) -> NewCrate {
        NewCrate {
            rustacean_id,
            code: code.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            description,
        }
    }

    pub fn create_multiple_crates(count: usize) -> Vec<Crate> {
        (1..=count)
            .map(|i| Self::create_crate_with_id(i as i32))
            .collect()
    }
}

/// Test data factory for creating mock roles
pub struct RoleFactory;

impl RoleFactory {
    pub fn create_admin_role() -> Role {
        Role {
            id: 1,
            code: RoleCode::Admin,
            name: "Administrator".to_string(),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    pub fn create_editor_role() -> Role {
        Role {
            id: 2,
            code: RoleCode::Editor,
            name: "Editor".to_string(),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    pub fn create_role_with_code(code: RoleCode) -> Role {
        Role {
            id: match code {
                RoleCode::Admin => 1,
                RoleCode::Editor => 2,
                RoleCode::Viewer => 3,
            },
            code: code.clone(),
            name: match code {
                RoleCode::Admin => "Administrator".to_string(),
                RoleCode::Editor => "Editor".to_string(),
                RoleCode::Viewer => "Viewer".to_string(),
            },
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    pub fn create_multiple_roles() -> Vec<Role> {
        vec![Self::create_admin_role(), Self::create_editor_role()]
    }
}

/// Common test assertions and utilities
pub struct TestAssertions;

impl TestAssertions {
    /// Assert that a DieselError is of NotFound variant
    pub fn assert_not_found_error(error: &DieselError) {
        match error {
            DieselError::NotFound => (),
            _ => panic!("Expected NotFound error, got: {:?}", error),
        }
    }

    /// Assert that a DieselError is a DatabaseError
    pub fn assert_database_error(error: &DieselError) {
        match error {
            DieselError::DatabaseError(_, _) => (),
            _ => panic!("Expected DatabaseError, got: {:?}", error),
        }
    }

    /// Assert that a vector contains a specific item
    pub fn assert_contains<T: PartialEq + std::fmt::Debug>(vec: &[T], item: &T) {
        assert!(
            vec.contains(item),
            "Vector does not contain expected item: {:?}",
            item
        );
    }

    /// Assert that a string contains a JSON field
    pub fn assert_json_contains_field(json_str: &str, field: &str) {
        assert!(
            json_str.contains(field),
            "JSON string does not contain field '{}': {}",
            field,
            json_str
        );
    }

    /// Assert that a string does not contain a JSON field
    pub fn assert_json_excludes_field(json_str: &str, field: &str) {
        assert!(
            !json_str.contains(field),
            "JSON string should not contain field '{}': {}",
            field,
            json_str
        );
    }
}

/// Mock configuration and setup utilities
pub struct MockSetup;

impl MockSetup {
    /// Create a HashMap for mock cache operations
    pub fn create_mock_cache() -> HashMap<String, String> {
        HashMap::new()
    }

    /// Generate a mock session ID
    pub fn generate_mock_session_id() -> String {
        "mock_session_id_123456789".to_string()
    }

    /// Create mock session key format
    pub fn create_session_key(session_id: &str) -> String {
        format!("sessions/{}", session_id)
    }

    /// Create mock Redis error
    pub fn create_redis_error(message: &str) -> String {
        format!("Redis error: {}", message)
    }

    /// Create mock database connection error
    pub fn create_db_error(message: &str) -> DieselError {
        DieselError::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new(message.to_string()),
        )
    }
}

/// Test validation utilities
pub struct ValidationHelpers;

impl ValidationHelpers {
    /// Validate email format (basic validation for tests)
    pub fn is_valid_email(email: &str) -> bool {
        email.contains('@') && email.contains('.') && !email.is_empty()
    }

    /// Validate version format (basic semver validation for tests)
    pub fn is_valid_version(version: &str) -> bool {
        let parts: Vec<&str> = version.split('.').collect();
        parts.len() >= 3 && parts.iter().take(3).all(|part| part.parse::<u32>().is_ok())
    }

    /// Validate username format
    pub fn is_valid_username(username: &str) -> bool {
        !username.is_empty() && username.len() >= 3 && username.len() <= 50
    }

    /// Validate crate code format
    pub fn is_valid_crate_code(code: &str) -> bool {
        !code.is_empty()
            && code.len() <= 64
            && code
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    }
}

/// Constants for testing
pub mod test_constants {
    pub const DEFAULT_LIMIT: i64 = 100;
    pub const SESSION_DURATION_SECONDS: i32 = 3 * 60 * 60; // 3 hours
    pub const MOCK_TIMESTAMP: i64 = 1640995200; // 2022-01-01 00:00:00 UTC
    pub const TEST_PASSWORD_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$VE0e2XSF+9Tz7/hYP8I9dQ$J4moa2MM3d6FRYiW6BHjbVa7SdKHHjhZXJ4pGSXnKl8";
    pub const SESSION_ID_LENGTH: usize = 128;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_factory_creates_valid_user() {
        let user = UserFactory::create_user();
        assert_eq!(user.id, 1);
        assert_eq!(user.username, "testuser");
        assert!(!user.password.is_empty());
    }

    #[test]
    fn test_rustacean_factory_creates_valid_rustacean() {
        let rustacean = RustaceanFactory::create_rustacean();
        assert_eq!(rustacean.id, 1);
        assert_eq!(rustacean.name, "John Doe");
        assert_eq!(rustacean.email, "john@example.com");
    }

    #[test]
    fn test_crate_factory_creates_valid_crate() {
        let crate_data = CrateFactory::create_crate();
        assert_eq!(crate_data.id, 1);
        assert_eq!(crate_data.rustacean_id, 1);
        assert_eq!(crate_data.code, "test_crate");
    }

    #[test]
    fn test_role_factory_creates_admin_role() {
        let role = RoleFactory::create_admin_role();
        assert_eq!(role.code, RoleCode::Admin);
        assert_eq!(role.name, "Administrator");
    }

    #[test]
    fn test_validation_helpers_email_validation() {
        assert!(ValidationHelpers::is_valid_email("test@example.com"));
        assert!(!ValidationHelpers::is_valid_email("invalid-email"));
        assert!(!ValidationHelpers::is_valid_email(""));
    }

    #[test]
    fn test_validation_helpers_version_validation() {
        assert!(ValidationHelpers::is_valid_version("1.0.0"));
        assert!(ValidationHelpers::is_valid_version("0.1.0"));
        assert!(!ValidationHelpers::is_valid_version("1.0"));
        assert!(!ValidationHelpers::is_valid_version("invalid"));
    }

    #[test]
    fn test_mock_setup_session_key_format() {
        let session_id = "test_session_123";
        let key = MockSetup::create_session_key(session_id);
        assert_eq!(key, "sessions/test_session_123");
    }

    #[test]
    fn test_create_multiple_rustaceans() {
        let rustaceans = RustaceanFactory::create_multiple_rustaceans(3);
        assert_eq!(rustaceans.len(), 3);
        assert_eq!(rustaceans[0].id, 1);
        assert_eq!(rustaceans[1].id, 2);
        assert_eq!(rustaceans[2].id, 3);
    }

    #[test]
    fn test_create_multiple_crates() {
        let crates = CrateFactory::create_multiple_crates(2);
        assert_eq!(crates.len(), 2);
        assert_eq!(crates[0].id, 1);
        assert_eq!(crates[1].id, 2);
    }
}
