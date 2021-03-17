use crate::result::Error;
use bytes::Bytes;
use futures::FutureExt as _;
use juniper::{
    http::{GraphQLBatchRequest, GraphQLRequest},
    ScalarValue,
};
use std::{collections::HashMap, str, sync::Arc};
use tokio::task;
use warp::{body, filters::BoxedFilter, http, query, Filter};

pub fn make_graphql_filter<Query, Mutation, Subscription, CtxT, S>(
    schema: juniper::RootNode<'static, Query, Mutation, Subscription, S>,
    context_extractor: BoxedFilter<(CtxT,)>,
) -> BoxedFilter<(http::Response<Vec<u8>>,)>
where
    Query: juniper::GraphQLTypeAsync<S, Context = CtxT> + Send + 'static,
    Query::TypeInfo: Send + Sync,
    Mutation: juniper::GraphQLTypeAsync<S, Context = CtxT> + Send + 'static,
    Mutation::TypeInfo: Send + Sync,
    Subscription: juniper::GraphQLSubscriptionType<S, Context = CtxT> + Send + 'static,
    Subscription::TypeInfo: Send + Sync,
    CtxT: Send + Sync + 'static,
    S: ScalarValue + Send + Sync + 'static,
{
    let schema = Arc::new(schema);
    let post_json_schema = schema.clone();
    let post_graphql_schema = schema.clone();

    let handle_post_json_request = move |context: CtxT, req: GraphQLBatchRequest<S>| {
        let schema = post_json_schema.clone();
        async move {
            let resp = req.execute(&schema, &context).await;

            Ok::<_, warp::Rejection>(build_response(
                serde_json::to_vec(&resp)
                    .map(|json| (json, resp.is_ok()))
                    .map_err(Into::into),
            ))
        }
    };
    let post_json_filter = warp::post()
        .and(context_extractor.clone())
        .and(body::json())
        .and_then(handle_post_json_request);

    let handle_post_graphql_request = move |context: CtxT, body: Bytes| {
        let schema = post_graphql_schema.clone();
        async move {
            let query = str::from_utf8(body.as_ref()).map_err(|e| {
                Error::InternalServerError(format!(
                    "Request body query is not a valid UTF-8 string: {}",
                    e
                ))
            })?;
            let req = GraphQLRequest::new(query.into(), None, None);

            let resp = req.execute(&schema, &context).await;

            Ok((serde_json::to_vec(&resp)?, resp.is_ok()))
        }
        .then(|res| async { Ok::<_, warp::Rejection>(build_response(res)) })
    };
    let post_graphql_filter = warp::post()
        .and(context_extractor.clone())
        .and(body::bytes())
        .and_then(handle_post_graphql_request);

    let handle_get_request = move |context: CtxT, mut qry: HashMap<String, String>| {
        let schema = schema.clone();
        async move {
            let req = GraphQLRequest::new(
                qry.remove("query").ok_or_else(|| {
                    Error::InternalServerError(format!(
                        "Missing GraphQL query string in query parameters"
                    ))
                })?,
                qry.remove("operation_name"),
                qry.remove("variables")
                    .map(|vs| serde_json::from_str(&vs))
                    .transpose()?,
            );

            let resp = req.execute(&schema, &context).await;

            Ok((serde_json::to_vec(&resp)?, resp.is_ok()))
        }
        .then(|res| async move { Ok::<_, warp::Rejection>(build_response(res)) })
    };
    let get_filter = warp::get()
        .and(context_extractor)
        .and(query::query())
        .and_then(handle_get_request);

    get_filter
        .or(post_json_filter)
        .unify()
        .or(post_graphql_filter)
        .unify()
        .boxed()
}

/// Error raised by `tokio_threadpool` if the thread pool has been shutdown.
///
/// Wrapper type is needed as inner type does not implement `warp::reject::Reject`.
#[derive(Debug)]
pub struct JoinError(task::JoinError);

impl warp::reject::Reject for JoinError {}

fn build_response(response: Result<(Vec<u8>, bool), Error>) -> http::Response<Vec<u8>> {
    match response {
        Ok((body, is_ok)) => http::Response::builder()
            .status(if is_ok { 200 } else { 400 })
            .header("content-type", "application/json")
            .body(body)
            .expect("response is valid"),
        Err(_) => http::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(Vec::new())
            .expect("status code is valid"),
    }
}
