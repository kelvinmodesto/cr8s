use crate::config::{CacheConn, PgConn};
use crate::models::RoleCode;
use crate::repositories::{RoleRepository, UserRepository};
use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, Response};
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[rocket::options("/<_route_args..>")]
pub fn options(_route_args: Option<std::path::PathBuf>) {
    // do nothing
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Append CORS headers in responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_raw_header("Access-Control-Allow-Origin", "*");
        res.set_raw_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
        res.set_raw_header("Access-Control-Allow-Headers", "*");
        res.set_raw_header("Access-Control-Allow-Credentials", "true");
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let session_header = req
            .headers()
            .get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(header_value) = session_header {
            let mut cache = match req.guard::<Connection<CacheConn>>().await {
                Outcome::Success(conn) => conn,
                Outcome::Error(_) => {
                    eprintln!("Failed to connect to Redis during authentication");
                    return Outcome::Error((Status::ServiceUnavailable, ()));
                }
                Outcome::Forward(status) => {
                    eprintln!("Redis connection forwarded with status {:?}", status);
                    return Outcome::Forward(status);
                }
            };

            let mut db = match req.guard::<Connection<PgConn>>().await {
                Outcome::Success(conn) => conn,
                Outcome::Error(_) => {
                    eprintln!("Failed to connect to Postgres during authentication");
                    return Outcome::Error((Status::ServiceUnavailable, ()));
                }
                Outcome::Forward(status) => {
                    eprintln!("Postgres connection forwarded with status {:?}", status);
                    return Outcome::Forward(status);
                }
            };

            let result = cache
                .get::<String, i32>(format!("sessions/{}", header_value[1]))
                .await;
            match result {
                Ok(user_id) => match UserRepository::find(&mut db, user_id).await {
                    Ok(user) => return Outcome::Success(user),
                    Err(err) => {
                        eprintln!("Failed to find user {}: {:?}", user_id, err);
                        return Outcome::Error((Status::Unauthorized, ()));
                    }
                },
                Err(err) => {
                    eprintln!("Failed to get session from Redis: {:?}", err);
                    return Outcome::Error((Status::Unauthorized, ()));
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
        let user = match req.guard::<User>().await {
            Outcome::Success(user) => user,
            Outcome::Error((status, _)) => {
                eprintln!("Failed to retrieve curent logged in user");
                return Outcome::Error((status, ()));
            }
            Outcome::Forward(status) => {
                eprintln!("User guard forwarded with status: {:?}", status);
                return Outcome::Forward(status);
            }
        };

        let mut db = match req.guard::<Connection<PgConn>>().await {
            Outcome::Success(conn) => conn,
            Outcome::Error(_) => {
                eprintln!("Failed to connect to Postgres for role check");
                return Outcome::Error((Status::ServiceUnavailable, ()));
            }
            Outcome::Forward(status) => {
                eprintln!("User guard forwarded with status: {:?}", status);
                return Outcome::Forward(status);
            }
        };

        match RoleRepository::find_by_user(&mut db, &user).await {
            Ok(roles) => {
                let is_editor = roles
                    .iter()
                    .any(|r| matches!(r.code, RoleCode::Admin | RoleCode::Editor));

                if is_editor {
                    return Outcome::Success(EditorUser(user));
                } else {
                    eprintln!("User {} lacks editor privileges", user.id);
                    return Outcome::Error((Status::Forbidden, ()));
                }
            }
            Err(err) => {
                eprintln!("Failed to fetch roles for user {}: {:?}", user.id, err);
                return Outcome::Error((Status::ServiceUnavailable, ()));
            }
        }
    }
}
