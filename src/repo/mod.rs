use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::*;

#[async_trait]
pub trait UserRepo {
    async fn by_id(&self, id: Uuid) -> Option<User>;
}
