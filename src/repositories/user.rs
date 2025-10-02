use crate::{
    models::{NewRole, NewUser, NewUserRole, Role, RoleCode, User, UserRole},
    repositories::RoleRepository,
    schema::{roles, users, users_roles},
};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_username(
        conn: &mut AsyncPgConnection,
        username: &String,
    ) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .get_result(conn)
            .await
    }

    pub async fn find_with_roles(
        conn: &mut AsyncPgConnection,
    ) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load::<User>(conn).await?;
        let result = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(conn)
            .await?
            .grouped_by(&users);
        Ok(users.into_iter().zip(result).collect())
    }

    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(conn).await
    }

    pub async fn create(
        conn: &mut AsyncPgConnection,
        new_user: NewUser,
        role_codes: Vec<RoleCode>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(conn)
            .await?;

        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(conn, &role_code).await {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    let name = role_code.to_string();
                    let new_role = NewRole {
                        code: role_code,
                        name,
                    };

                    let role = RoleRepository::create(conn, new_role).await?;
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };

            diesel::insert_into(users_roles::table)
                .values(new_user_role)
                .get_result::<UserRole>(conn)
                .await?;
        }
        Ok(user)
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users_roles::table.filter(users_roles::user_id.eq(id)))
            .execute(conn)
            .await?;
        diesel::delete(users::table.find(id)).execute(conn).await
    }
}
