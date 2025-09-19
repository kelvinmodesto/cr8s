use reqwest::StatusCode;
use reqwest::blocking::Client;
use rocket::form::validate::Contains;
use rocket::serde::json::serde_json::json;
use serde_json::Value;

pub mod common;

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let rustacean1: Value = common::create_test_rustacean(&client);
    let rustacean2: Value = common::create_test_rustacean(&client);
    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().contains(&rustacean1));
    assert!(json.as_array().contains(&rustacean2));
    common::delete_test_rustacean(&client, rustacean1);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
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
    let client = Client::new();
    let rustacean: Value = common::create_test_rustacean(&client);
    let response = client
        .get(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
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
fn test_update_rustacean() {
    let client = Client::new();
    let rustacean: Value = common::create_test_rustacean(&client);
    let response = client
        .put(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
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
    let client = Client::new();
    let rustacean: Value = common::create_test_rustacean(&client);
    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
