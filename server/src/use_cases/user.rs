use crate::result::Result;
use crate::POOL;
use chrono::Utc;
use uuid::Uuid;

use crate::repo::UserRepo;

struct UserUseCase<'a, U>
where
    U: UserRepo,
{
    user_repo: &'a U,
}

impl<'a, U> UserUseCase<'a, U>
where
    U: UserRepo,
{
    pub fn new(user_repo: &'a U) -> Self {
        Self { user_repo }
    }

    pub fn register(&self, body: ()) -> Result<()> {
        Ok(())
    }
}
