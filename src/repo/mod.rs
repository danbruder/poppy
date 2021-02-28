use async_trait::async_trait;
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
}
