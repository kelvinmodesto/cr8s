use reqwest::StatusCode;
use reqwest::blocking::Client;
use rocket::serde::json::serde_json::json;
use serde_json::Value;

pub static APP_HOST: &str = "http://127.0.0.1:8000";

pub fn create_test_rustacean(client: &Client) -> Value {
    let response = client
        .post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({ "name": "Monkey D. Luffy", "email": "luffy@mugiwara.d" }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client
        .delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn create_test_crate(client: &Client, rustacean: Value) {
    let response = client
        .post(format!("{}/crates", APP_HOST))
        .json(&json!({ "name": "Thousand Sunny", "code": "mugiwara", "rustacean_id": rustacean["id"]}))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

pub fn delete_test_crate(client: &Client, a_crate: Value) {
    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
