use std::env::var;
use std::str::FromStr;

use crate::utils::auth::hash_password;
use crate::utils::email::HtmlMailer;
use chrono::{Datelike, Utc};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use tera::{Context, Tera};

use crate::models::{NewUser, RoleCode};
use crate::repositories::{CrateRepository, RoleRepository, UserRepository};

fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html").expect("Cannot load template engine")
}

async fn load_db_connection() -> AsyncPgConnection {
    let database_url =
        std::env::var("DATABASE_URL").expect("Cannot retrieve DB URL from environment");
    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannot connect to Postgres")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut connection = load_db_connection().await;

    let hash_pwd = hash_password(password).unwrap();
    let new_user = NewUser {
        username,
        password: hash_pwd,
    };
    let role_codes_list = role_codes
        .iter()
        .map(|r| RoleCode::from_str(r.as_str()).unwrap())
        .collect();
    let user = UserRepository::create(&mut connection, new_user, role_codes_list)
        .await
        .unwrap();
    println!("User created {:?}", user);

    let roles = RoleRepository::find_by_user(&mut connection, &user)
        .await
        .unwrap();
    println!("Roles assigned {:?}", roles);
}

pub async fn list_users() {
    let mut connection = load_db_connection().await;
    let users = UserRepository::find_with_roles(&mut connection)
        .await
        .unwrap();
    for user in users {
        println!("{:?}", user);
    }
}

pub async fn delete_user(id: i32) {
    let mut connection = load_db_connection().await;
    UserRepository::delete(&mut connection, id).await.unwrap();

    println!("User with id {:?} were deleted", id);
}

pub async fn digest_send(email: String, hours_since: i32) {
    let mut connection = load_db_connection().await;
    let tera = load_template_engine();

    let crates = CrateRepository::find_since(&mut connection, hours_since)
        .await
        .unwrap();
    if crates.len() > 0 {
        let year = Utc::now().year();
        let mut context = Context::new();

        context.insert("crates", &crates);
        context.insert("year", &year);

        let smtp_host = var("SMTP_HOST").expect("Cannot load SMTP host from environment");
        let smtp_username =
            var("SMTP_USERNAME").expect("Cannot load SMTP username from environment");
        let smtp_password =
            var("SMTP_PASSWORD").expect("Cannot load SMTP password from environment");

        let mailer = HtmlMailer {
            template_engine: tera,
            smtp_host,
            smtp_username,
            smtp_password,
        };
        mailer.send(email, "email/digest.html", context).unwrap();
    }
}
