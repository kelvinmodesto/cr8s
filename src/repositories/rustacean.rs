use crate::{
    models::{NewRustacean, Rustacean},
    schema::rustaceans,
};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub async fn view(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(conn).await
    }

    pub async fn find(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).load(conn).await
    }

    pub async fn create(
        conn: &mut AsyncPgConnection,
        new_rustacean: NewRustacean,
    ) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(conn)
            .await
    }

    pub async fn update(
        conn: &mut AsyncPgConnection,
        id: i32,
        rustacean: Rustacean,
    ) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email),
            ))
            .get_result(conn)
            .await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id))
            .execute(conn)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{NewRustacean, Rustacean};

    // Mock data helpers
    fn create_test_rustacean() -> Rustacean {
        Rustacean {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    fn create_new_test_rustacean() -> NewRustacean {
        NewRustacean {
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
        }
    }

    #[test]
    fn test_new_rustacean_creation() {
        let new_rustacean = create_new_test_rustacean();
        assert_eq!(new_rustacean.name, "Jane Smith");
        assert_eq!(new_rustacean.email, "jane@example.com");
    }

    #[test]
    fn test_rustacean_fields() {
        let rustacean = create_test_rustacean();
        assert_eq!(rustacean.id, 1);
        assert_eq!(rustacean.name, "John Doe");
        assert_eq!(rustacean.email, "john@example.com");
        assert!(rustacean.created_at.and_utc().timestamp() >= 0);
    }
}
