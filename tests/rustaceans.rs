use reqwest::StatusCode;
use reqwest::blocking::Client;
use serde_json::{Value, json};
pub mod common;

#[cfg(test)]
mod rustaceans_happy_path {
    use super::*;

    #[test]
    fn test_get_rustaceans() {
        let client = common::get_client_with_logged_in_editor();
        let rustacean1: Value = common::create_test_rustacean(&client);
        let rustacean2: Value = common::create_test_rustacean(&client);

        let response = client
            .get(format!("{}/rustaceans", common::APP_HOST))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let json: Value = response.json().unwrap();
        println!("{}", json.as_array().unwrap().contains(&rustacean1));
        assert!(json.as_array().unwrap().contains(&rustacean1));
        assert!(json.as_array().unwrap().contains(&rustacean2));

        common::delete_test_rustacean(&client, rustacean1);
        common::delete_test_rustacean(&client, rustacean2);
    }

    #[test]
    fn test_create_rustacean() {
        let client = common::get_client_with_logged_in_editor();
        let response = client
            .post(format!("{}/rustaceans", common::APP_HOST))
            .json(&json!({ "name": "foo bar", "email": "foo@bar.com" }))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let rustacean: Value = response.json().unwrap();
        assert_eq!(
            rustacean,
            json!({ "id": rustacean["id"],"name": "foo bar", "email": "foo@bar.com", "created_at": rustacean["created_at"] })
        );

        common::delete_test_rustacean(&client, rustacean);
    }

    #[test]
    fn test_view_rustacean() {
        let client = common::get_client_with_logged_in_editor();
        let rustacean: Value = common::create_test_rustacean(&client);

        let response = client
            .get(format!(
                "{}/rustaceans/{}",
                common::APP_HOST,
                rustacean["id"]
            ))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let rustacean: Value = response.json().unwrap();
        assert_eq!(
            rustacean,
            json!({ "id": rustacean["id"],"name": "Monkey D. Luffy", "email": "luffy@mugiwara.d", "created_at": rustacean["created_at"] })
        );

        common::delete_test_rustacean(&client, rustacean);
    }

    #[test]
    fn test_view_rustacean_not_found() {
        let client = common::get_client_with_logged_in_editor();
        let rustacean: Value = common::create_test_rustacean(&client);

        let response = client
            .get(format!("{}/rustaceans/9999", common::APP_HOST))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        common::delete_test_rustacean(&client, rustacean);
    }

    #[test]
    fn test_update_rustacean() {
        let client = common::get_client_with_logged_in_editor();
        let rustacean: Value = common::create_test_rustacean(&client);

        let response = client
            .put(format!(
                "{}/rustaceans/{}",
                common::APP_HOST,
                rustacean["id"]
            ))
            .json(&json!({ "name": "Monkey D. Luffy", "email": "nika@mugiwara.d" }))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let rustacean: Value = response.json().unwrap();
        assert_eq!(
            rustacean,
            json!({ "id": rustacean["id"],"name": "Monkey D. Luffy", "email": "nika@mugiwara.d", "created_at": rustacean["created_at"] })
        );

        common::delete_test_rustacean(&client, rustacean);
    }

    #[test]
    fn test_delete_rustacean() {
        let client = common::get_client_with_logged_in_editor();
        let rustacean: Value = common::create_test_rustacean(&client);

        let response = client
            .delete(format!(
                "{}/rustaceans/{}",
                common::APP_HOST,
                rustacean["id"]
            ))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}

#[cfg(test)]
mod rustaceans_error_cases {

    use super::*;

    #[test]
    fn test_viewer_cannot_get_rustaceans() {
        let client = Client::new();

        let rustacean1: Value = common::create_test_rustacean(&client);
        let rustacean2: Value = common::create_test_rustacean(&client);

        let response = client
            .get(format!("{}/rustaceans", common::APP_HOST))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        common::delete_test_rustacean(&client, rustacean1);
        common::delete_test_rustacean(&client, rustacean2);
    }

    #[test]
    fn test_create_rustacean() {
        let client = common::get_client_with_logged_in_editor();
        let response = client
            .post(format!("{}/rustaceans", common::APP_HOST))
            .json(&json!({ "name": "foo bar", "email": "foo@bar.com" }))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let rustacean: Value = response.json().unwrap();
        assert_eq!(
            rustacean,
            json!({ "id": rustacean["id"],"name": "foo bar", "email": "foo@bar.com", "created_at": rustacean["created_at"] })
        );

        common::delete_test_rustacean(&client, rustacean);
    }

    #[test]
    fn test_view_rustacean() {
        let client = common::get_client_with_logged_in_editor();
        let rustacean: Value = common::create_test_rustacean(&client);

        let response = client
            .get(format!(
                "{}/rustaceans/{}",
                common::APP_HOST,
                rustacean["id"]
            ))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let rustacean: Value = response.json().unwrap();
        assert_eq!(
            rustacean,
            json!({ "id": rustacean["id"],"name": "Monkey D. Luffy", "email": "luffy@mugiwara.d", "created_at": rustacean["created_at"] })
        );

        common::delete_test_rustacean(&client, rustacean);
    }

    #[test]
    fn test_view_rustacean_not_found() {
        let client = common::get_client_with_logged_in_editor();
        let rustacean: Value = common::create_test_rustacean(&client);

        let response = client
            .get(format!("{}/rustaceans/9999", common::APP_HOST))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        common::delete_test_rustacean(&client, rustacean);
    }

    #[test]
    fn test_update_rustacean() {
        let client = common::get_client_with_logged_in_editor();
        let rustacean: Value = common::create_test_rustacean(&client);

        let response = client
            .put(format!(
                "{}/rustaceans/{}",
                common::APP_HOST,
                rustacean["id"]
            ))
            .json(&json!({ "name": "Monkey D. Luffy", "email": "nika@mugiwara.d" }))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let rustacean: Value = response.json().unwrap();
        assert_eq!(
            rustacean,
            json!({ "id": rustacean["id"],"name": "Monkey D. Luffy", "email": "nika@mugiwara.d", "created_at": rustacean["created_at"] })
        );

        common::delete_test_rustacean(&client, rustacean);
    }

    #[test]
    fn test_delete_rustacean() {
        let client = common::get_client_with_logged_in_editor();
        let rustacean: Value = common::create_test_rustacean(&client);

        let response = client
            .delete(format!(
                "{}/rustaceans/{}",
                common::APP_HOST,
                rustacean["id"]
            ))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
