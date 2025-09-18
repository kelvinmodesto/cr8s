use rocket_db_pools::Database;

mod config;
mod models;
mod repositories;
mod routes;
mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                routes::rustaceans::get_rustaceans,
                routes::rustaceans::view_rustacean,
                routes::rustaceans::create_rustacean,
                routes::rustaceans::update_rustacean,
                routes::rustaceans::delete_rustacean,
            ],
        )
        .attach(config::postgres::PgConn::init())
        .launch()
        .await;
}
