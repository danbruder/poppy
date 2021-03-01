use serde::Deserialize;

use crate::repo::UserRepo;
use crate::result::Result;

#[derive(Clone, new)]
pub struct UserUseCase<U>
where
    U: UserRepo,
{
    user_repo: U,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

impl<U> UserUseCase<U>
where
    U: UserRepo,
{
    pub async fn register(&self, body: &RegisterRequest) -> Result<String> {
        let existing_user = self.user_repo.by_email(&body.email).await?;

        let hashed = bcrypt::hash(&body.password, bcrypt::DEFAULT_COST)?;

        // Check user against hashed
        panic!("todo");
        Ok("hello".into())
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_register() {}
}
