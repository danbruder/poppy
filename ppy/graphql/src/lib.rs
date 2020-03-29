pub use juniper_subscriptions;
pub use juniper_warp;

use juniper::{FieldResult, GraphQLInputObject, GraphQLObject, RootNode};

#[derive(Clone)]
pub struct Context {}
impl juniper::Context for Context {}

#[derive(GraphQLObject)]
pub struct User {
    pub id: i32,
    pub name: String,
}

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

pub struct Subscription;

#[juniper::graphql_subscription(Context = Context)]
impl Subscription {}

pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, Subscription)
}
