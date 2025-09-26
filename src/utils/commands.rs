use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::models::{NewRole, NewUser};
use crate::repositories::{RoleRepository, UserRepository};

async fn load_db_connection() -> AsyncPgConnection {
    let database_url =
        std::env::var("DATABASE_URL").expect("Cannot retrieve DB URL from environment");
    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannot connect to Postgres")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut connection = load_db_connection().await;
    let new_user = NewUser { username, password };

    let user = UserRepository::create(&mut connection, new_user, role_codes)
        .await
        .unwrap();
    println!("User created {:?}", user);

    let roles = RoleRepository::find_by_user(&mut connection, user)
        .await
        .unwrap();
    println!("Roles assigned {:?}", roles);
}

pub async fn list_users() {
    let mut connection = load_db_connection().await;
}

pub async fn delete_user(id: i32) {
    let mut connection = load_db_connection().await;
}
