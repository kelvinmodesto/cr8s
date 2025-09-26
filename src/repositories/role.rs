use crate::{
    models::{NewRole, Role, User, UserRole},
    schema::roles,
};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct RoleRepository;

impl RoleRepository {
    pub async fn find_by_ids(
        conn: &mut AsyncPgConnection,
        ids: Vec<i32>,
    ) -> QueryResult<Vec<Role>> {
        roles::table.filter(roles::id.eq_any(ids)).load(conn).await
    }

    pub async fn find_by_code(conn: &mut AsyncPgConnection, code: String) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code)).first(conn).await
    }

    pub async fn find_by_user(conn: &mut AsyncPgConnection, user: User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(&user)
            .get_results::<UserRole>(conn)
            .await?;

        let role_ids: Vec<i32> = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();
        Self::find_by_ids(conn, role_ids).await
    }

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
