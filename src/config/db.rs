use rocket_db_pools::Database;

#[derive(Database)]
#[database("postgres")]
pub struct PgConn(rocket_db_pools::diesel::PgPool);
