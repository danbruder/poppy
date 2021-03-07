use crate::entities::Photo;
use crate::repo::PhotoRepo;
use crate::result::Result;
use bytes::Bytes;
use futures::Stream;
use std::pin::Pin;
use uuid::Uuid;

#[derive(Clone, new)]
pub struct PhotoUseCase<U>
where
    U: PhotoRepo,
{
    photo_repo: U,
}

type FileStream = Pin<Box<dyn Stream<Item = Result<Option<Bytes>>> + Send>>;

pub struct UploadInput {
    pub name: String,
    pub stream: FileStream,
}

impl<U> PhotoUseCase<U>
where
    U: PhotoRepo,
{
    pub async fn list(&self) -> Result<Vec<Photo>> {
        self.photo_repo.list().await
    }

    pub async fn upload(&self, input: UploadInput) -> Result<Photo> {
        let file = self.photo_repo.store_file(input.bytes).await?;
        let photo = Photo::new(Uuid::new_v4().to_string(), file.path);
        self.photo_repo.create(&photo).await?;

        Ok(photo)
    }
}
