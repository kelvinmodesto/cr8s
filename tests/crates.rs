use reqwest::StatusCode;
use serde_json::{Value, json};

pub mod common;

#[cfg(test)]
mod crates_happy_path {
    use super::*;

    #[test]
    fn test_create_crate() {
        let client = common::get_client_with_logged_in_editor();
        let rustacean = common::create_test_rustacean(&client);

        let response = client
            .post(format!("{}/crates", common::APP_HOST))
            .json(&json!({
                "rustacean_id": rustacean["id"],
                "code": "mugiwara",
                "name": "Thousand Sunny",
                "version": "0.10.3",
                "description": "A crew of pirates that use to free people around the world"
            }))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let a_crate: Value = response.json().unwrap();
        assert_eq!(
            a_crate,
            json!({
                "id": a_crate["id"],
                "name": "Thousand Sunny",
                "code": "mugiwara",
                "version": "0.10.3",
                "description": "A crew of pirates that use to free people around the world" ,
                "rustacean_id": a_crate["rustacean_id"],
                "created_at": a_crate["created_at"]
            })
        );

        common::delete_test_crate(&client, a_crate);
        common::delete_test_rustacean(&client, rustacean);
    }

    #[test]
    fn test_get_crates() {
        let client = common::get_client_with_logged_in_editor();
        let rustacean: Value = common::create_test_rustacean(&client);
        let a_crate: Value = common::create_test_crate(&client, &rustacean);
        let b_crate: Value = common::create_test_crate(&client, &rustacean);

        let response = client
            .get(format!("{}/crates", common::APP_HOST))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let json: Value = response.json().unwrap();
        assert!(json.as_array().unwrap().contains(&a_crate));
        assert!(json.as_array().unwrap().contains(&b_crate));

        common::delete_test_crate(&client, a_crate);
        common::delete_test_crate(&client, b_crate);
        common::delete_test_rustacean(&client, rustacean);
    }

    #[test]
    fn test_view_crate() {
        let client = common::get_client_with_logged_in_editor();
        let rustacean: Value = common::create_test_rustacean(&client);
        let a_crate: Value = common::create_test_crate(&client, &rustacean);
        let response = client
            .get(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let crate_response: Value = response.json().unwrap();
        assert_eq!(
            crate_response,
            json!({
                "id": crate_response["id"],
                "name": "Thousand Sunny",
                "code": "mugiwara",
                "version": "0.10.3",
                "description": "A crew of pirates that use to free people around the world" ,
                "rustacean_id": crate_response["rustacean_id"],
                "created_at": crate_response["created_at"]
            })
        );
        common::delete_test_crate(&client, a_crate);
        common::delete_test_rustacean(&client, rustacean);
    }

    #[test]
    fn test_update_crate() {
        let client = common::get_client_with_logged_in_editor();
        let rustacean: Value = common::create_test_rustacean(&client);
        let a_crate: Value = common::create_test_crate(&client, &rustacean);

        let response = client
            .put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
            .json(&json!({
                "name": "Thousand Sunny",
                "code": "nika",
                "version": "0.10.4",
                "description": "A crew of pirates that use to free people around the world" ,
                "rustacean_id": rustacean["id"],
            }))
            .send()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let a_crate: Value = response.json().unwrap();
        assert_eq!(
            a_crate,
            json!({
                "id": a_crate["id"],
                "name": "Thousand Sunny",
                "code": "nika",
                "version": "0.10.4",
                "description": "A crew of pirates that use to free people around the world" ,
                "rustacean_id": a_crate["rustacean_id"],
                "created_at": a_crate["created_at"],
            })
        );
        common::delete_test_crate(&client, a_crate);
        common::delete_test_rustacean(&client, rustacean);
    }

    #[test]
    fn test_delete_crate() {
        let client = common::get_client_with_logged_in_editor();
        let rustacean: Value = common::create_test_rustacean(&client);
        let a_crate: Value = common::create_test_crate(&client, &rustacean);

        let response = client
            .delete(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        common::delete_test_rustacean(&client, rustacean);
    }
}

mod crates_error_cases {
    use reqwest::blocking::Client;

    use super::*;

    #[test]
    fn test_create_crate_error() {
        let client = Client::new();
        let client_admin = common::get_client_with_logged_in_admin();
        let rustacean = common::create_test_rustacean(&client_admin);

        let response = client
            .post(format!("{}/crates", common::APP_HOST))
            .json(&json!({
                "rustacean_id": rustacean["id"],
                "code": "mugiwara",
                "name": "Thousand Sunny",
                "version": "0.10.3",
                "description": "A crew of pirates that use to free people around the world"
            }))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        common::delete_test_rustacean(&client_admin, rustacean);
    }

    #[test]
    fn test_get_crates_error() {
        let client = Client::new();
        let client_admin = common::get_client_with_logged_in_admin();

        let rustacean: Value = common::create_test_rustacean(&client_admin);
        let a_crate: Value = common::create_test_crate(&client_admin, &rustacean);
        let b_crate: Value = common::create_test_crate(&client_admin, &rustacean);

        let response = client
            .get(format!("{}/crates", common::APP_HOST))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        common::delete_test_crate(&client_admin, a_crate);
        common::delete_test_crate(&client_admin, b_crate);
        common::delete_test_rustacean(&client_admin, rustacean);
    }

    #[test]
    fn test_view_crate_error() {
        let client = Client::new();
        let client_admin = common::get_client_with_logged_in_admin();

        let rustacean: Value = common::create_test_rustacean(&client_admin);
        let a_crate: Value = common::create_test_crate(&client_admin, &rustacean);

        let response = client
            .get(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
            .send()
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        common::delete_test_crate(&client_admin, a_crate);
        common::delete_test_rustacean(&client_admin, rustacean);
    }

    #[test]
    fn test_update_crate_error() {
        let client_admin = common::get_client_with_logged_in_admin();
        let client = Client::new();

        let rustacean: Value = common::create_test_rustacean(&client_admin);
        let a_crate: Value = common::create_test_crate(&client_admin, &rustacean);

        let response = client
            .put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
            .json(&json!({
                "name": "Thousand Sunny",
                "code": "nika",
                "version": "0.10.4",
                "description": "A crew of pirates that use to free people around the world" ,
                "rustacean_id": rustacean["id"],
            }))
            .send()
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        common::delete_test_crate(&client_admin, a_crate);
        common::delete_test_rustacean(&client_admin, rustacean);
    }
}
