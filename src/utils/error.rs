use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::response::status::Custom;
use serde_json::{Value, json};
use std::error::Error;

pub fn handle_diesel_error(e: DieselError) -> Custom<Value> {
    match e {
        DieselError::NotFound => Custom(Status::NotFound, json!("Not found")),
        DieselError::DatabaseError(_, _) => {
            rocket::error!("Database error: {}", e);
            Custom(Status::InternalServerError, json!("Database error"))
        }
        _ => {
            rocket::error!("Diesel error: {}", e);
            Custom(Status::InternalServerError, json!("Error"))
        }
    }
}

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}
