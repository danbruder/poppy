use crate::entities::Photo;
use crate::repo::PhotoRepo;
use crate::result::Result;

#[derive(Clone)]
pub struct PhotoUseCase<U>
where
    U: PhotoRepo,
{
    photo_repo: U,
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
}
