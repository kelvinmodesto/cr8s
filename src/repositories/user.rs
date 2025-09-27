use crate::{
    models::{NewRole, NewUser, NewUserRole, Role, User, UserRole},
    repositories::RoleRepository,
    schema::{roles, users, users_roles},
};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct UserRepository;

impl UserRepository {
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
    pub async fn view(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(conn).await
    }

    pub async fn find(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<User>> {
        users::table.limit(limit).load(conn).await
    }

    pub async fn create(
        conn: &mut AsyncPgConnection,
        new_user: NewUser,
        role_codes: Vec<String>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(conn)
            .await?;

        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(conn, role_code.to_owned()).await {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    let new_role = NewRole {
                        code: role_code.to_owned(),
                        name: role_code.to_owned(),
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

    pub async fn update(conn: &mut AsyncPgConnection, id: i32, user: User) -> QueryResult<User> {
        diesel::update(users::table.find(id))
            .set((
                users::username.eq(user.username),
                users::password.eq(user.password),
            ))
            .get_result(conn)
            .await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users_roles::table.filter(users_roles::user_id.eq(id)))
            .execute(conn)
            .await?;
        diesel::delete(users::table.find(id)).execute(conn).await
    }
}
