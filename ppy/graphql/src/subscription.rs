use futures::{Future, FutureExt as _, Stream};
use juniper::FieldError;

use crate::context::Context;
use crate::user::User;

#[juniper::graphql_object(Context = Context)]
pub struct Subscription;
