use crate::entities::Photo;
use crate::repo::PhotoRepo;
use crate::result::Result;
use bytes::Bytes;

#[derive(Clone)]
pub struct PhotoUseCase<U>
where
    U: PhotoRepo,
{
    photo_repo: U,
}

pub struct UploadInput {
    name: String,
    file: Bytes,
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

        //self.photo_repo.list().await
    }
}
