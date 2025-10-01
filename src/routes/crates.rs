use crate::config::PgConn;
use crate::models::{Crate, EditorUser, NewCrate, User};
use crate::repositories::CrateRepository;
use crate::utils::error::{handle_diesel_error, server_error};
use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{Json, json};
use rocket_db_pools::Connection;
use serde_json::Value;

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<PgConn>, _user: User) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, 100)
        .await
        .map(|cr| json!(cr))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(
    mut db: Connection<PgConn>,
    id: i32,
    _user: User,
) -> Result<Value, Custom<Value>> {
    CrateRepository::view(&mut db, id)
        .await
        .map(|cr| json!(cr))
        .map_err(handle_diesel_error)
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    mut db: Connection<PgConn>,
    new_crate: Json<NewCrate>,
    _user: EditorUser,
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
    _user: EditorUser,
) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut db, id, crate_data.into_inner())
        .await
        .map(|cr| json!(cr))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(
    mut db: Connection<PgConn>,
    id: i32,
    _user: EditorUser,
) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}
