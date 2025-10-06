use crate::models::{NewRole, NewUser, NewUserRole, Role, RoleCode, User, UserRole};
use crate::repositories::RoleRepository;
use crate::schema::{roles, users, users_roles};
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{NewUser, RoleCode, User};

    // Mock data helpers
    fn create_test_user() -> User {
        User {
            id: 1,
            username: "testuser".to_string(),
            password: "hashed_password".to_string(),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    fn create_new_test_user() -> NewUser {
        NewUser {
            username: "newuser".to_string(),
            password: "new_hashed_password".to_string(),
        }
    }

    #[test]
    fn test_new_user_creation() {
        let new_user = create_new_test_user();
        assert_eq!(new_user.username, "newuser");
        assert_eq!(new_user.password, "new_hashed_password");
    }

    #[test]
    fn test_user_fields() {
        let user = create_test_user();
        assert_eq!(user.id, 1);
        assert_eq!(user.username, "testuser");
        assert_eq!(user.password, "hashed_password");
        assert!(user.created_at.and_utc().timestamp() >= 0);
    }

    #[test]
    fn test_role_code_variants_in_user_context() {
        let role_codes = vec![RoleCode::Admin, RoleCode::Editor];

        for role_code in role_codes {
            let new_user = NewUser {
                username: format!("user_{:?}", role_code).to_lowercase(),
                password: "password".to_string(),
            };

            assert!(!new_user.username.is_empty());
            assert!(!new_user.password.is_empty());
        }
    }
}
