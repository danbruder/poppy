pub mod user;
pub use user::*;

use crate::repo::UserRepo;

struct UseCaseCollection<'a, U: UserRepo> {
    user: user::UserUseCase<'a, U>,
}
