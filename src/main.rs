use futures::FutureExt as _;
use std::{env, sync::Arc};
#[macro_use]
extern crate lazy_static;

use std::convert::Infallible;

use dotenv::dotenv;
use juniper_graphql_ws::ConnectionConfig;
use juniper_warp::{playground_filter, subscriptions::serve_graphql_ws};
use serde_derive::Serialize;
use warp::{http::Response, http::StatusCode, Filter};
use warp::{Rejection, Reply};

mod db;
mod entities;
mod graphql;
mod repo;
mod result;
mod use_cases;

#[cfg(test)]
mod integration_tests;

use graphql::{schema, Context};

lazy_static! {
    //pub static ref POOL: SqlitePool = db::setup();
    static ref DOMAIN: String = "localhost".into();
}

#[tokio::main] // or #[tokio::main]
async fn main() {
    dotenv().ok();

    env::set_var("RUST_LOG", "app");

    // Initialize resources
    env_logger::init();
    result::init_error_tracking();
    //db::migrate().await;

    let routes = get_routes();

    log::info!("Listening on 127.0.0.1:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

pub fn get_routes() -> impl warp::Filter<Extract = impl Reply> + Clone {
    let log = warp::log("warp_subscriptions");
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-length")
        .allow_header("content-type")
        .allow_header("date")
        .allow_methods(vec!["GET", "OPTIONS", "POST", "DELETE"]);

    // Create a connection pool
    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body("<html><h1>juniper_subscriptions demo</h1><div>visit <a href=\"/playground\">graphql playground</a></html>".to_string())
    });

    let qm_schema = schema();
    let qm_state = warp::any().map(move || Context {});
    let qm_graphql_filter = juniper_warp::make_graphql_filter(qm_schema, qm_state.boxed());

    let root_node = Arc::new(schema());

    let subs_route = warp::path("subscriptions")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let root_node = root_node.clone();
            ws.on_upgrade(move |websocket| async move {
                serve_graphql_ws(websocket, root_node, ConnectionConfig::new(Context {}))
                    .map(|r| {
                        if let Err(e) = r {
                            println!("Websocket error: {}", e);
                        }
                    })
                    .await
            })
        })
        .map(|reply| warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws"));

    let graphql_route = warp::post()
        .and(warp::path("graphql"))
        .and(qm_graphql_filter);

    let playground_route = warp::get()
        .and(warp::path("playground"))
        .and(playground_filter("/graphql", Some("/subscriptions")));

    let register_route = warp::post()
        .and(warp::path("register"))
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .and_then(|body: use_cases::RegisterRequest| async move {
            let repo = db::DbUserRepo {};
            let user = use_cases::UserUseCase::new(&repo);

            user.register(&body)
                .await
                .map(|session_id| {
                    let reply = warp::reply::json(&"success");
                    warp::reply::with_header(
                        reply,
                        "Set-Cookie",
                        format!(
                            "session={}; Domain={}; Secure; HttpOnly",
                            session_id,
                            DOMAIN.as_str()
                        ),
                    )
                })
                .map_err(|_| warp::reject::not_found())
        });

    subs_route
        .or(graphql_route)
        .or(playground_route)
        .or(homepage)
        .or(register_route)
        .recover(handle_rejection)
        .with(log)
        .with(cors)
}

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
