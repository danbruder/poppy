use crate::entities::Photo;
use crate::repo::PhotoRepo;
use crate::result::Result;

#[derive(Clone, new)]
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
    pub async fn list(&self) -> Result<Vec<Photo>> {
        self.photo_repo.list().await
    }

    pub async fn create(&self, path: &str) -> Result<Photo> {
        let photo = Photo::new(path);
        self.photo_repo.create(&photo).await?;
        Ok(photo)
    }
}
