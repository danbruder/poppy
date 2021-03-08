use async_trait::async_trait;
use bytes::Bytes;

use futures::Stream;
use sqlx::sqlite::SqlitePool;
use std::pin::Pin;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::entities::*;
use crate::result::*;

lazy_static! {
    pub static ref POOL: SqlitePool = setup();
}

pub fn setup() -> SqlitePool {
    SqlitePool::connect_lazy("/data/poppy.db").expect("Could not connect to database")
}

pub async fn migrate() {
    let _ = tokio::fs::File::create("/data/poppy.db").await.unwrap();
    tokio::fs::create_dir("/data/files").await.unwrap();

    sqlx::migrate!()
        .run(&*POOL)
        .await
        .expect("Failed to run migrations");
}

#[derive(Clone)]
pub struct UserRepo;

#[async_trait]
impl crate::repo::UserRepo for UserRepo {
    async fn by_id(&self, id: Uuid) -> Option<User> {
        None
    }

    async fn by_email(&self, email: &str) -> Result<Option<User>> {
        Ok(None)
    }
}

#[derive(Clone)]
pub struct PhotoRepo;

#[async_trait]
impl crate::repo::PhotoRepo for PhotoRepo {
    async fn list(&self) -> Result<Vec<Photo>> {
        let photos = sqlx::query_as!(Photo, "SELECT id, uri FROM photo")
            .fetch_all(&*POOL)
            .await?;
        Ok(photos)
    }

    async fn create(&self, input: &Photo) -> Result<()> {
        sqlx::query_as!(
            Photo,
            "INSERT INTO photo (id, uri) VALUES (?1, ?2)",
            input.id,
            input.uri
        )
        .execute(&*POOL)
        .await?;

        Ok(())
    }
}
