#[derive(rocket_db_pools::Database)]
#[database("redis")]
pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);
