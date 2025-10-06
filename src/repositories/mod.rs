mod crate_repository;
mod role;
mod rustacean;
mod user;

pub use crate_repository::*;
pub use role::*;
pub use rustacean::*;
pub use user::*;

// Repository traits for mocking
use crate::models::{
    Crate, NewCrate, NewRole, NewRustacean, NewUser, Role, RoleCode, Rustacean, User, UserRole,
};
use diesel::result::QueryResult;
use diesel_async::AsyncPgConnection;

#[cfg_attr(test, mockall::automock)]
pub trait RustaceanRepositoryTrait {
    async fn view(&mut self, conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean>;
    async fn find(
        &mut self,
        conn: &mut AsyncPgConnection,
        limit: i64,
    ) -> QueryResult<Vec<Rustacean>>;
    async fn create(
        &mut self,
        conn: &mut AsyncPgConnection,
        new_rustacean: NewRustacean,
    ) -> QueryResult<Rustacean>;
    async fn update(
        &mut self,
        conn: &mut AsyncPgConnection,
        id: i32,
        rustacean: Rustacean,
    ) -> QueryResult<Rustacean>;
    async fn delete(&mut self, conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>;
}

#[cfg_attr(test, mockall::automock)]
pub trait CrateRepositoryTrait {
    async fn view(&mut self, conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate>;
    async fn find(&mut self, conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Crate>>;
    async fn find_since(
        &mut self,
        conn: &mut AsyncPgConnection,
        hours_since: i32,
    ) -> QueryResult<Vec<Crate>>;
    async fn create(
        &mut self,
        conn: &mut AsyncPgConnection,
        new_crate: NewCrate,
    ) -> QueryResult<Crate>;
    async fn update(
        &mut self,
        conn: &mut AsyncPgConnection,
        id: i32,
        existent_crate: Crate,
    ) -> QueryResult<Crate>;
    async fn delete(&mut self, conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>;
}

#[cfg_attr(test, mockall::automock)]
pub trait RoleRepositoryTrait {
    async fn find_by_ids(
        &mut self,
        conn: &mut AsyncPgConnection,
        ids: Vec<i32>,
    ) -> QueryResult<Vec<Role>>;
    async fn find_by_code(
        &mut self,
        conn: &mut AsyncPgConnection,
        code: &RoleCode,
    ) -> QueryResult<Role>;
    async fn find_by_user(
        &mut self,
        conn: &mut AsyncPgConnection,
        user: &User,
    ) -> QueryResult<Vec<Role>>;
    async fn create(
        &mut self,
        conn: &mut AsyncPgConnection,
        new_role: NewRole,
    ) -> QueryResult<Role>;
}

#[cfg_attr(test, mockall::automock)]
pub trait UserRepositoryTrait {
    async fn find_by_username(
        &mut self,
        conn: &mut AsyncPgConnection,
        username: &String,
    ) -> QueryResult<User>;
    async fn find_with_roles(
        &mut self,
        conn: &mut AsyncPgConnection,
    ) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>>;
    async fn find(&mut self, conn: &mut AsyncPgConnection, id: i32) -> QueryResult<User>;
    async fn create(
        &mut self,
        conn: &mut AsyncPgConnection,
        new_user: NewUser,
        role_codes: Vec<RoleCode>,
    ) -> QueryResult<User>;
    async fn delete(&mut self, conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize>;
}
