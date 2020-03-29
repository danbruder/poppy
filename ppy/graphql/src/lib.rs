pub use juniper_subscriptions;
pub use juniper_warp;

use juniper::{GraphQLInputObject, GraphQLObject, RootNode};

#[derive(Clone)]
pub struct Context {}

impl juniper::Context for Context {}

#[derive(GraphQLObject)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(GraphQLInputObject)]
pub struct NewUser {
    pub name: String,
}

// Field resolvers implementation
pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn users(id: i32) -> Vec<User> {
        vec![User {
            id,
            name: "User Name".into(),
        }]
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {}

pub struct Subscription;

#[juniper::graphql_subscription(Context = Context)]
impl Subscription {}

pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, Subscription)
}
