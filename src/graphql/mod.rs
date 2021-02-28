use juniper::{graphql_object, EmptyMutation, EmptySubscription, GraphQLObject, RootNode};

use crate::result::Result;

#[derive(Clone)]
pub struct Context {}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    pub fn photos() -> Result<Vec<Photo>> {
        let photos = vec![];
        Ok(photos)
    }
}

// Photos

#[derive(GraphQLObject)]
struct Photo {
    url: String,
}
