use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use std::env;
use uuid::Uuid;

use crate::entities::*;
use crate::repo::*;
use crate::result::*;

lazy_static! {
    pub static ref POOL: SqlitePool = setup();
}

pub fn setup() -> SqlitePool {
    SqlitePool::connect_lazy(&env::var("DATABASE_URL").expect("DATABASE_URL required"))
        .expect("Could not connect to database")
}

pub async fn migrate() {
    sqlx::migrate!()
        .run(&*POOL)
        .await
        .expect("Failed to run migrations");
}

struct DbUserRepo {}

#[async_trait]
impl UserRepo for DbUserRepo {
    async fn by_id(&self, id: Uuid) -> Option<User> {
        None
    }

    async fn by_email(&self, email: &str) -> Result<Option<User>> {
        Ok(None)
    }
}
