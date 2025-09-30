use crate::config::{CacheConn, PgConn};
use crate::repositories::UserRepository;
use crate::utils::auth::{Credentials, authorize_user};
use crate::utils::error::server_error;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{Json, Value, json};
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;

//TODO: Put cache logic into a repository(aka sessionRepository)
#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db: Connection<PgConn>,
    mut cache: Connection<CacheConn>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    let user = UserRepository::find_by_username(&mut db, &credentials.username)
        .await
        .map_err(|e| server_error(e.into()))?;
    let session_id = authorize_user(&user, credentials.into_inner())
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credentials")))?;

    cache
        .set_ex::<String, i32, ()>(format!("sessions/{}", session_id), user.id, 3 * 60 * 60)
        .await
        .map_err(|e| server_error(e.into()))?;

    Ok(json!({
        "token": session_id,
    }))
}
