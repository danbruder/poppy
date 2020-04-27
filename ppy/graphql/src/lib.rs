mod context;
mod mutation;
mod query;
mod subscription;
mod user;

pub use juniper_subscriptions;
pub use juniper_warp;

pub use context::Context;
use mutation::Mutation;
use query::Query;
use subscription::Subscription;

pub type Schema = juniper::RootNode<'static, Query, Mutation, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, Subscription)
}
