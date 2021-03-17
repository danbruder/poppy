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
    pub async fn register(&self, _body: &RegisterRequest) -> Result<String> {
        Ok("".into())
    }
}
