use juniper::{graphql_object, EmptySubscription, RootNode};

use crate::db::{PhotoRepo, UserRepo};
use crate::entities::Photo;
use crate::result::Result;
use crate::use_cases::{PhotoUseCase, UserUseCase};

#[derive(Clone)]
pub struct Context {
    pub photo: PhotoUseCase<PhotoRepo>,
    pub user: UserUseCase<UserRepo>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new() -> Self {
        Self {
            photo: PhotoUseCase::new(PhotoRepo),
            user: UserUseCase::new(UserRepo),
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

    fn public_url(&self) -> String {
        self.public_url()
    }
}
