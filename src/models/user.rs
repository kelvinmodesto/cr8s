use crate::config::{CacheConn, PgConn};
use crate::models::RoleCode;
use crate::repositories::{RoleRepository, UserRepository};
use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub password: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Authorization: Bearer SESSION_ID_128_CHARACTERS_LONG
        let session_header = req
            .headers()
            .get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(header_value) = session_header {
            // TODO: Use try_outcome for Graceful Shutdown
            // use rocket::outcome::try_outcome;
            // try_outcome!();

            let mut cache = req
                .guard::<Connection<CacheConn>>()
                .await
                .expect("Can not connect to Redis in Request guard");
            let mut db = req
                .guard::<Connection<PgConn>>()
                .await
                .expect("Can not connect to Postgres in Request guard");
            let result = cache
                .get::<String, i32>(format!("sessions/{}", header_value[1]))
                .await;
            if let Ok(user_id) = result {
                if let Ok(user) = UserRepository::find(&mut db, user_id).await {
                    return Outcome::Success(user);
                }
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}

pub struct EditorUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = req
            .guard::<User>()
            .await
            .expect("Cannot retrieve current logged in user");
        let mut db = req
            .guard::<Connection<PgConn>>()
            .await
            .expect("Cannot connect to Postgres in request guard");

        if let Ok(roles) = RoleRepository::find_by_user(&mut db, &user).await {
            let is_editor = roles
                .iter()
                .any(|r| matches!(r.code, RoleCode::Admin | RoleCode::Editor));

            if is_editor {
                return Outcome::Success(EditorUser(user));
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}
