extern crate c8rs;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                c8rs::routes::auth::login,
                c8rs::routes::rustaceans::get_rustaceans,
                c8rs::routes::rustaceans::view_rustacean,
                c8rs::routes::rustaceans::create_rustacean,
                c8rs::routes::rustaceans::update_rustacean,
                c8rs::routes::rustaceans::delete_rustacean,
                c8rs::routes::crates::get_crates,
                c8rs::routes::crates::view_crate,
                c8rs::routes::crates::create_crate,
                c8rs::routes::crates::update_crate,
                c8rs::routes::crates::delete_crate,
            ],
        )
        .attach(c8rs::config::CacheConn::init())
        .attach(c8rs::config::PgConn::init())
        .launch()
        .await;
}
