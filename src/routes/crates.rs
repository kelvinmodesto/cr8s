use crate::config::postgres::PgConn;
use crate::models::{Crate, NewCrate};
use crate::repositories::CrateRepository;
use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{Json, json};
use rocket_db_pools::Connection;
use serde_json::Value;

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<PgConn>) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, 100)
        .await
        .map(|cr| json!(cr))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(mut db: Connection<PgConn>, id: i32) -> Result<Value, Custom<Value>> {
    CrateRepository::view(&mut db, id)
        .await
        .map(|cr| json!(cr))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    mut db: Connection<PgConn>,
    new_crate: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner())
        .await
        .map(|cr| Custom(Status::Created, json!(cr)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::put("/crates/<id>", format = "json", data = "<crate_data>")]
pub async fn update_crate(
    mut db: Connection<PgConn>,
    id: i32,
    crate_data: Json<Crate>,
) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut db, id, crate_data.into_inner())
        .await
        .map(|cr| json!(cr))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<PgConn>, id: i32) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}
