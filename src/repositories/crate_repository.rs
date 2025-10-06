use crate::models::{Crate, NewCrate};
use crate::schema::crates;

use diesel::dsl::{IntervalDsl, now};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct CrateRepository;

impl CrateRepository {
    pub async fn view(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(conn).await
    }

    pub async fn find(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table.limit(limit).get_results(conn).await
    }

    pub async fn find_since(
        conn: &mut AsyncPgConnection,
        hours_since: i32,
    ) -> QueryResult<Vec<Crate>> {
        crates::table
            .filter(crates::created_at.ge(now - hours_since.hours()))
            .load(conn)
            .await
    }

    pub async fn create(conn: &mut AsyncPgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result(conn)
            .await
    }

    pub async fn update(
        conn: &mut AsyncPgConnection,
        id: i32,
        existent_crate: Crate,
    ) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::rustacean_id.eq(existent_crate.rustacean_id),
                crates::name.eq(existent_crate.name),
                crates::code.eq(existent_crate.code),
                crates::version.eq(existent_crate.version),
                crates::description.eq(existent_crate.description),
            ))
            .get_result(conn)
            .await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(conn).await
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{Crate, NewCrate};

    // Mock data helpers
    fn create_test_crate() -> Crate {
        Crate {
            id: 1,
            rustacean_id: 1,
            code: "test_crate".to_string(),
            name: "Test Crate".to_string(),
            version: "0.1.0".to_string(),
            description: Some("A test crate".to_string()),
            created_at: chrono::DateTime::from_timestamp(1640995200, 0)
                .unwrap()
                .naive_utc(),
        }
    }

    fn create_new_test_crate() -> NewCrate {
        NewCrate {
            rustacean_id: 1,
            code: "new_test_crate".to_string(),
            name: "New Test Crate".to_string(),
            version: "0.2.0".to_string(),
            description: Some("A new test crate".to_string()),
        }
    }

    #[test]
    fn test_new_crate_creation() {
        let new_crate = create_new_test_crate();
        assert_eq!(new_crate.rustacean_id, 1);
        assert_eq!(new_crate.code, "new_test_crate");
        assert_eq!(new_crate.name, "New Test Crate");
        assert_eq!(new_crate.version, "0.2.0");
        assert_eq!(new_crate.description, Some("A new test crate".to_string()));
    }

    #[test]
    fn test_crate_fields() {
        let test_crate = create_test_crate();
        assert_eq!(test_crate.id, 1);
        assert_eq!(test_crate.rustacean_id, 1);
        assert_eq!(test_crate.code, "test_crate");
        assert_eq!(test_crate.name, "Test Crate");
        assert_eq!(test_crate.version, "0.1.0");
        assert_eq!(test_crate.description, Some("A test crate".to_string()));
        assert!(test_crate.created_at.and_utc().timestamp() >= 0);
    }
}
