use reqwest::{StatusCode, blocking::Client};
use serde_json::{Value, json};

pub mod common;

#[cfg(test)]
mod auth_happy_path {
    use super::*;

    #[test]
    fn test_sucessful_login() {
        common::create_user_by_cli();
        let client = Client::new();
        let response = client
            .post(format!("{}/login", common::APP_HOST))
            .json(&json!({
                "username": "test_admin",
                "password": "1234",
            }))
            .send()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let json: Value = response.json().unwrap();
        assert!(json.get("token").is_some());
        assert_eq!(json["token"].as_str().unwrap().len(), 128);
    }

    #[test]
    fn test_unsucessful_login() {
        common::create_user_by_cli();
        let client = Client::new();
        let response = client
            .post(format!("{}/login", common::APP_HOST))
            .json(&json!({
                "username": "test_admin",
                "password": "12345",
            }))
            .send()
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_me() {
        let client = common::get_client_with_logged_in_admin();

        let response = client
            .get(format!("{}/me", common::APP_HOST))
            .send()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let json: Value = response.json().unwrap();

        assert!(json.get("id").is_some());
        assert!(json.get("username").is_some());
        assert!(json.get("created_at").is_some());
        assert_eq!(json["username"], "test_admin");
        assert!(json.get("password").is_none());
    }
}

#[cfg(test)]
mod auth_error_cases {
    use super::*;

    #[test]
    fn test_unsucessful_login() {
        common::create_user_by_cli();
        let client = Client::new();
        let response = client
            .post(format!("{}/login", common::APP_HOST))
            .json(&json!({
                "username": "test_admin",
                "password": "wrong_password",
            }))
            .send()
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_non_existent_username() {
        common::create_user_by_cli();
        let client = Client::new();
        let response = client
            .post(format!("{}/login", common::APP_HOST))
            .json(&json!({
                "username": "nobody",
                "password": "1234",
            }))
            .send()
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_me_error() {
        let client = Client::new();

        let response = client
            .get(format!("{}/me", common::APP_HOST))
            .send()
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
