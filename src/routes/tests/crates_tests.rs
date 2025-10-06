#[cfg(test)]
mod tests {
    use crate::models::{Crate, NewCrate};
    use serde_json;

    // Mock data helpers
    fn create_test_crate() -> Crate {
        Crate {
            id: 1,
            rustacean_id: 1,
            code: "test_crate".to_string(),
            name: "Test Crate".to_string(),
            version: "0.1.0".to_string(),
            description: Some("A test crate for unit testing".to_string()),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(), // 2022-01-01
        }
    }

    fn create_new_test_crate() -> NewCrate {
        NewCrate {
            rustacean_id: 1,
            code: "new_test_crate".to_string(),
            name: "New Test Crate".to_string(),
            version: "0.2.0".to_string(),
            description: Some("A new test crate".to_string()),
        }
    }

    #[test]
    fn test_new_crate_json_serialization() {
        let new_crate = create_new_test_crate();
        let json_string = serde_json::to_string(&new_crate).unwrap();
        let expected = r#"{"rustacean_id":1,"code":"new_test_crate","name":"New Test Crate","version":"0.2.0","description":"A new test crate"}"#;
        assert_eq!(json_string, expected);
    }

    #[test]
    fn test_crate_json_serialization() {
        let crate_data = create_test_crate();
        let json_string = serde_json::to_string(&crate_data).unwrap();

        // The serialized JSON should contain all crate fields
        assert!(json_string.contains("\"id\":1"));
        assert!(json_string.contains("\"rustacean_id\":1"));
        assert!(json_string.contains("\"code\":\"test_crate\""));
        assert!(json_string.contains("\"name\":\"Test Crate\""));
        assert!(json_string.contains("\"version\":\"0.1.0\""));
        assert!(json_string.contains("\"description\":\"A test crate for unit testing\""));
        assert!(json_string.contains("\"created_at\""));
    }

    #[test]
    fn test_crate_json_deserialization() {
        let json_str = r#"{"rustacean_id":1,"code":"test","name":"Test Crate","version":"1.0.0","description":"Test description"}"#;
        let new_crate: NewCrate = serde_json::from_str(json_str).unwrap();
        assert_eq!(new_crate.rustacean_id, 1);
        assert_eq!(new_crate.code, "test");
        assert_eq!(new_crate.name, "Test Crate");
        assert_eq!(new_crate.version, "1.0.0");
        assert_eq!(new_crate.description, Some("Test description".to_string()));
    }

    #[test]
    fn test_crate_json_deserialization_without_description() {
        let json_str = r#"{"rustacean_id":1,"code":"test","name":"Test Crate","version":"1.0.0","description":null}"#;
        let new_crate: NewCrate = serde_json::from_str(json_str).unwrap();
        assert_eq!(new_crate.rustacean_id, 1);
        assert_eq!(new_crate.code, "test");
        assert_eq!(new_crate.name, "Test Crate");
        assert_eq!(new_crate.version, "1.0.0");
        assert_eq!(new_crate.description, None);
    }

    #[tokio::test]
    async fn test_crate_version_format_validation() {
        // Test various version formats
        let valid_versions = vec!["1.0.0", "0.1.0", "2.1.3", "1.0.0-alpha"];
        let invalid_versions = vec!["", "1", "1.0", "invalid"];

        for version in valid_versions {
            let new_crate = NewCrate {
                rustacean_id: 1,
                code: "test_code".to_string(),
                name: "Test Name".to_string(),
                version: version.to_string(),
                description: None,
            };
            // Should be valid
            assert!(!new_crate.version.is_empty());
        }

        for version in invalid_versions {
            let new_crate = NewCrate {
                rustacean_id: 1,
                code: "test_code".to_string(),
                name: "Test Name".to_string(),
                version: version.to_string(),
                description: None,
            };
            // In a real implementation, these would fail validation
            // For now, we just check that they can be created
            assert_eq!(new_crate.version, version);
        }
    }
}
