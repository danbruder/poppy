use crate::repo::UserRepo;
use crate::result::Result;

pub struct UserUseCase<'a, U>
where
    U: UserRepo,
{
    user_repo: &'a U,
}

pub struct RegisterRequest<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

impl<'a, U> UserUseCase<'a, U>
where
    U: UserRepo,
{
    pub fn new(user_repo: &'a U) -> Self {
        Self { user_repo }
    }

    pub async fn register(&self, body: &RegisterRequest<'_>) -> Result<()> {
        let existing_user = self.user_repo.by_email(body.email).await?;

        let hashed = bcrypt::hash(&body.password, bcrypt::DEFAULT_COST)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_register() {}
}
