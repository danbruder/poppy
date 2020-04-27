use juniper::FieldResult;

use crate::context::Context;
use crate::user::User;

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn login() -> FieldResult<User> {
        Ok(User {
            id: 1,
            name: "User Name".into(),
        })
    }
}
