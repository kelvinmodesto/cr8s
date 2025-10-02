use crate::{models::RoleCode, schema::*};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, Identifiable)]
pub struct Role {
    pub id: i32,
    pub code: RoleCode,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name=roles)]
pub struct NewRole {
    pub code: RoleCode,
    pub name: String,
}
