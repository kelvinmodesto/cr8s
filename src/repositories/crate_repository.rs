use crate::{
    models::{Crate, NewCrate},
    schema::crates,
};

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct CrateRepository;

impl CrateRepository {
    pub async fn findOne(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(conn).await
    }

    pub async fn find(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table.limit(limit).get_results(conn).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result(conn)
            .await
    }

    pub async fn update(conn: &mut AsyncPgConnection, existent_crate: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(existent_crate.id))
            .set((
                crates::rustacean_id.eq(existent_crate.rustacean_id),
                crates::name.eq(existent_crate.name),
                crates::code.eq(existent_crate.code),
                crates::version.eq(existent_crate.version),
                crates::description.eq(existent_crate.description),
            ))
            .get_result(conn)
            .await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(conn).await
    }
}
