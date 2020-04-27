use crate::context::Context;
use crate::user::User;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn users() -> Vec<User> {
        vec![User {
            id: 1,
            name: "User Name".into(),
        }]
    }
}
