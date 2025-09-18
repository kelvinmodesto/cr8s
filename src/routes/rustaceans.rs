use crate::config::postgres::PgConn;
use crate::models::{NewRustacean, Rustacean};
use crate::repositories::RustaceanRepository;
use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{Json, json};
use rocket_db_pools::Connection;
use serde_json::Value;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<PgConn>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find(&mut db, 100)
        .await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::get("/rustaceans/<id>")]
pub async fn view_rustacean(mut db: Connection<PgConn>, id: i32) -> Result<Value, Custom<Value>> {
    RustaceanRepository::view(&mut db, id)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    mut db: Connection<PgConn>,
    new_rustacean: Json<NewRustacean>,
) -> Result<Custom<Value>, Custom<Value>> {
    RustaceanRepository::create(&mut db, new_rustacean.into_inner())
        .await
        .map(|rustacean| Custom(Status::Created, json!(rustacean)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    mut db: Connection<PgConn>,
    id: i32,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::update(&mut db, id, rustacean.into_inner())
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    mut db: Connection<PgConn>,
    id: i32,
) -> Result<NoContent, Custom<Value>> {
    RustaceanRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}
