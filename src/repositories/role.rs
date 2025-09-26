use crate::{
    models::{NewRole, Role},
    schema::roles,
};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct RoleRepository;

impl RoleRepository {
    pub async fn view(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Role> {
        roles::table.find(id).get_result(conn).await
    }

    pub async fn find(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Role>> {
        roles::table.limit(limit).load(conn).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result(conn)
            .await
    }

    pub async fn update(conn: &mut AsyncPgConnection, id: i32, role: Role) -> QueryResult<Role> {
        diesel::update(roles::table.find(id))
            .set((roles::code.eq(role.code), roles::name.eq(role.name)))
            .get_result(conn)
            .await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(roles::table.find(id)).execute(conn).await
    }
}
