use async_trait::async_trait;
use bytes::Bytes;
use uuid::Uuid;

use crate::entities::*;
use crate::result::Result;

#[async_trait]
pub trait UserRepo {
    async fn by_id(&self, id: Uuid) -> Option<User>;
    async fn by_email(&self, email: &str) -> Result<Option<User>>;
}

#[async_trait]
pub trait PhotoRepo {
    async fn list(&self) -> Result<Vec<Photo>>;
    async fn store_file(&self, name: String, bytes: Bytes) -> Result<()>;
    async fn create(&self, input: &Photo) -> Result<()>;
}
