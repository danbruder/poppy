use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};

#[derive(Clone)]
pub struct Context {}

impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    pub fn hello() -> String {
        "world".into()
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}
