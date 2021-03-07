use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use std::pin::Pin;
use uuid::Uuid;

use crate::entities::*;
use crate::result::Result;

type FileStream = Pin<Box<dyn Stream<Item = Result<Option<Bytes>>> + Send>>;

#[async_trait]
pub trait UserRepo: Clone {
    async fn by_id(&self, id: Uuid) -> Option<User>;
    async fn by_email(&self, email: &str) -> Result<Option<User>>;
}

#[async_trait]
pub trait PhotoRepo: Clone {
    async fn list(&self) -> Result<Vec<Photo>>;
    async fn store_file(&self, filestream: FileStream) -> Result<File>;
    async fn create(&self, input: &Photo) -> Result<()>;
}
