use rocket::http::Status;
use rocket::response::status::Custom;
use serde_json::{Value, json};
use std::error::Error;

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}
