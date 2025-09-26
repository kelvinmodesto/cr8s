use crate::{
    models::{NewUser, User},
    schema::users,
};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct UserRepository;

impl UserRepository {
    pub async fn view(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(conn).await
    }

    pub async fn find(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<User>> {
        users::table.limit(limit).load(conn).await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(conn)
            .await
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
        diesel::delete(users::table.find(id)).execute(conn).await
    }
}
