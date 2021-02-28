use derive_more::{AsRef, Deref, From, Into};
use juniper::{graphql_object, EmptySubscription, GraphQLObject, RootNode};

use crate::db::PhotoRepo;
use crate::entities::Photo;
use crate::result::Result;
use crate::use_cases::PhotoUseCase;

#[derive(Clone)]
pub struct Context {
    photo: PhotoUseCase<PhotoRepo>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new() -> Self {
        Self {
            photo: PhotoUseCase::new(PhotoRepo {}),
        }
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

pub struct Query;
pub struct Mutation;

#[graphql_object(context = Context)]
impl Query {
    pub async fn photos(context: &Context) -> Result<Vec<Photo>> {
        context.photo.list().await
    }
}

#[graphql_object(context = Context)]
impl Mutation {
    pub fn hello() -> String {
        "world".into()
    }
}

// Photos

#[graphql_object]
impl Photo {
    fn uri(&self) -> &str {
        self.uri()
    }
}
