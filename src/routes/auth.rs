use crate::utils::error::server_error;
use crate::{config::postgres::PgConn, repositories::UserRepository};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rocket::response::status::Custom;
use rocket::serde::json::{Json, Value, json};
use rocket_db_pools::Connection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}
#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db: Connection<PgConn>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    UserRepository::find_by_username(&mut db, &credentials.username)
        .await
        .map(|user| {
            let argon2 = Argon2::default();
            let db_hash = PasswordHash::new(&user.password).unwrap();
            let result = argon2.verify_password(credentials.password.as_bytes(), &db_hash);
            if result.is_ok() {
                return json!("Sucess");
            }
            json!("Unauthorized")
        })
        .map_err(|e| server_error(e.into()))
}
