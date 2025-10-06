#[cfg(test)]
mod tests {
    use crate::models::{NewRustacean, Rustacean};
    use serde_json;

    // Mock data helpers
    fn create_test_rustacean() -> Rustacean {
        Rustacean {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(), // 2022-01-01
        }
    }

    fn create_new_test_rustacean() -> NewRustacean {
        NewRustacean {
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
        }
    }

    #[test]
    fn test_new_rustacean_json_serialization() {
        let new_rustacean = create_new_test_rustacean();
        let json_string = serde_json::to_string(&new_rustacean).unwrap();
        let expected = r#"{"name":"Jane Smith","email":"jane@example.com"}"#;
        assert_eq!(json_string, expected);
    }

    #[test]
    fn test_rustacean_json_serialization() {
        let rustacean = create_test_rustacean();
        let json_string = serde_json::to_string(&rustacean).unwrap();

        // The serialized JSON should contain id, name, email, and created_at
        assert!(json_string.contains("\"id\":1"));
        assert!(json_string.contains("\"name\":\"John Doe\""));
        assert!(json_string.contains("\"email\":\"john@example.com\""));
        assert!(json_string.contains("\"created_at\""));
    }

    #[test]
    fn test_rustacean_json_deserialization() {
        let json_str = r#"{"name":"Test User","email":"test@example.com"}"#;
        let rustacean: NewRustacean = serde_json::from_str(json_str).unwrap();
        assert_eq!(rustacean.name, "Test User");
        assert_eq!(rustacean.email, "test@example.com");
    }
}
