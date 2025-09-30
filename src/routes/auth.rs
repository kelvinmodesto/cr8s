use crate::utils::auth::{Credentials, authorize_user};
use crate::utils::error::server_error;
use crate::{config::postgres::PgConn, repositories::UserRepository};
use rocket::response::status::Custom;
use rocket::serde::json::{Json, Value, json};
use rocket_db_pools::Connection;

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db: Connection<PgConn>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    UserRepository::find_by_username(&mut db, &credentials.username)
        .await
        .map(|user| {
            if let Ok(token) = authorize_user(&user, credentials.into_inner()) {
                return json!(token);
            }
            json!("Unauthorized")
        })
        .map_err(|e| server_error(e.into()))
}
