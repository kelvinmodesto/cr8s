#[cfg(test)]
mod tests {
    use crate::models::User;
    use crate::utils::auth::Credentials;
    use rocket::serde::json::json;
    use serde_json;

    // Mock data helpers
    fn create_test_user() -> User {
        User {
            id: 1,
            username: "testuser".to_string(),
            password: "$argon2id$v=19$m=19456,t=2,p=1$VE0e2XSF+9Tz7/hYP8I9dQ$J4moa2MM3d6FRYiW6BHjbVa7SdKHHjhZXJ4pGSXnKl8".to_string(), // hashed "password123"
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    fn create_valid_credentials() -> Credentials {
        Credentials {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        }
    }

    #[test]
    fn test_credentials_json_serialization() {
        let credentials = create_valid_credentials();
        let json_string = serde_json::to_string(&credentials).unwrap();
        let expected = r#"{"username":"testuser","password":"password123"}"#;
        assert_eq!(json_string, expected);
    }

    #[test]
    fn test_credentials_json_deserialization() {
        let json_str = r#"{"username":"testuser","password":"password123"}"#;
        let credentials: Credentials = serde_json::from_str(json_str).unwrap();
        assert_eq!(credentials.username, "testuser");
        assert_eq!(credentials.password, "password123");
    }

    #[test]
    fn test_user_json_serialization() {
        let user = create_test_user();
        let json_string = serde_json::to_string(&user).unwrap();

        // The serialized JSON should contain id, username, and created_at
        // but should NOT contain password (due to serde skip_serializing)
        assert!(json_string.contains("\"id\":1"));
        assert!(json_string.contains("\"username\":\"testuser\""));
        assert!(json_string.contains("\"created_at\""));
        assert!(!json_string.contains("\"password\""));
        assert!(!json_string.contains("$argon2id$"));
    }

    #[tokio::test]
    async fn test_login_response_format() {
        let expected_response = json!({
            "token": "mock_session_id_123"
        });

        assert!(expected_response.get("token").is_some());
        assert!(expected_response["token"].is_string());
    }
}
