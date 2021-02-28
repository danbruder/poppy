use async_trait::async_trait;
use bytes::Bytes;
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

pub struct DbUserRepo {}

#[async_trait]
impl UserRepo for DbUserRepo {
    async fn by_id(&self, id: Uuid) -> Option<User> {
        None
    }

    async fn by_email(&self, email: &str) -> Result<Option<User>> {
        Ok(None)
    }
}

#[derive(Clone)]
pub struct PhotoRepo {}

#[async_trait]
impl crate::repo::PhotoRepo for PhotoRepo {
    async fn list(&self) -> Result<Vec<Photo>> {
        let photos = sqlx::query_as!(Photo, "SELECT uri FROM photo")
            .fetch_all(&*POOL)
            .await?;
        Ok(photos)
    }

    async fn store_file(&self, bytes: Bytes) -> Result<File> {
        let id = Uuid::new_v4();
        let mut file = File::create(format!("data/files/{}", id)).await?;
        file.write_all(b"hello, world!").await?;
    }
}
