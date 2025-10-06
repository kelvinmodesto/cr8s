use crate::{
    models::{NewRole, Role, RoleCode, User, UserRole},
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

    pub async fn find_by_code(conn: &mut AsyncPgConnection, code: &RoleCode) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code)).first(conn).await
    }

    pub async fn find_by_user(conn: &mut AsyncPgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(&user)
            .get_results::<UserRole>(conn)
            .await?;

        let role_ids: Vec<i32> = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();
        Self::find_by_ids(conn, role_ids).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result(conn)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{NewRole, Role, RoleCode};

    // Mock data helpers
    fn create_test_role() -> Role {
        Role {
            id: 1,
            code: RoleCode::Admin,
            name: "Administrator".to_string(),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    fn create_new_test_role() -> NewRole {
        NewRole {
            code: RoleCode::Editor,
            name: "Content Editor".to_string(),
        }
    }

    #[test]
    fn test_new_role_creation() {
        let new_role = create_new_test_role();
        assert_eq!(new_role.code, RoleCode::Editor);
        assert_eq!(new_role.name, "Content Editor");
    }

    #[test]
    fn test_role_fields() {
        let role = create_test_role();
        assert_eq!(role.id, 1);
        assert_eq!(role.code, RoleCode::Admin);
        assert_eq!(role.name, "Administrator");
        assert!(role.created_at.and_utc().timestamp() >= 0);
    }

    #[test]
    fn test_role_code_variants() {
        let admin_role = NewRole {
            code: RoleCode::Admin,
            name: "Admin".to_string(),
        };
        let editor_role = NewRole {
            code: RoleCode::Editor,
            name: "Editor".to_string(),
        };

        assert_eq!(admin_role.code, RoleCode::Admin);
        assert_eq!(editor_role.code, RoleCode::Editor);
    }
}
