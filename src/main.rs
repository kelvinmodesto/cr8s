use rocket_db_pools::Database;

mod config;
mod models;
mod repositories;
mod routes;
mod schema;
mod utils;

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
                routes::crates::get_crates,
                routes::crates::view_crate,
                routes::crates::create_crate,
                routes::crates::update_crate,
                routes::crates::delete_crate,
            ],
        )
        .attach(config::postgres::PgConn::init())
        .launch()
        .await;
}
