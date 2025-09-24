use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name=users_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}
