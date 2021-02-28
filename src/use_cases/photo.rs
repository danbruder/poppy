use crate::entities::Photo;
use crate::repo::PhotoRepo;
use crate::result::Result;
use bytes::Bytes;
use uuid::Uuid;

#[derive(Clone)]
pub struct PhotoUseCase<U>
where
    U: PhotoRepo,
{
    photo_repo: U,
}

pub struct UploadInput {
    pub name: String,
    pub file: Bytes,
}

impl<U> PhotoUseCase<U>
where
    U: PhotoRepo,
{
    pub fn new(photo_repo: U) -> Self {
        Self { photo_repo }
    }

    pub async fn list(&self) -> Result<Vec<Photo>> {
        self.photo_repo.list().await
    }

    pub async fn upload(&self, input: UploadInput) -> Result<Photo> {
        self.photo_repo.store_file(input.name, input.file).await?;
        let photo = Photo {
            id: Uuid::new_v4().to_string(),
            uri: "name.png".into(),
        };
        self.photo_repo.create(&photo).await?;

        Ok(photo)
    }
}
