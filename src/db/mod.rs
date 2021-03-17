use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

use crate::config;
use crate::entities::*;
use crate::result::*;

lazy_static! {
    pub static ref POOL: SqlitePool = setup();
}

pub fn setup() -> SqlitePool {
    let url = config::get_database_url();
    let url = url.to_str();
    let url = url.unwrap();
    SqlitePool::connect_lazy(&url).expect("Could not connect to database")
}

pub async fn migrate() {
    sqlx::migrate!()
        .run(&*POOL)
        .await
        .expect("Failed to run migrations");
}

#[derive(Clone)]
pub struct UserRepo;

#[async_trait]
impl crate::repo::UserRepo for UserRepo {
    async fn by_id(&self, _id: Uuid) -> Option<User> {
        None
    }

    async fn by_email(&self, _email: &str) -> Result<Option<User>> {
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
