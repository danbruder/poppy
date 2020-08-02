use std::{pin::Pin, sync::Arc};

use futures::{Future, FutureExt as _};
use ppy_graphql::juniper_subscriptions::{self, Coordinator};
use ppy_graphql::juniper_warp::{self, playground_filter, subscriptions::graphql_subscriptions};
use warp::{http::Response, Filter};

pub async fn run() {
    ::std::env::set_var("RUST_LOG", "warp_subscriptions");
    env_logger::init();

    let log = warp::log("warp_server");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body("<html><h1>juniper_subscriptions demo</h1><div>visit <a href=\"/playground\">graphql playground</a></html>".to_string())
    });

    let qm_schema = ppy_graphql::schema();
    let qm_state = warp::any().map(move || ppy_graphql::Context {});
    let qm_graphql_filter = juniper_warp::make_graphql_filter(qm_schema, qm_state.boxed());

    let sub_state = warp::any().map(move || ppy_graphql::Context {});
    let coordinator = Arc::new(juniper_subscriptions::Coordinator::new(
        ppy_graphql::schema(),
    ));

    log::info!("Listening on 127.0.0.1:8888");

    let routes = (warp::path("subscriptions")
        .and(warp::ws())
        .and(sub_state.clone())
        .and(warp::any().map(move || Arc::clone(&coordinator)))
        .map(
            |ws: warp::ws::Ws,
             ctx: ppy_graphql::Context,
             coordinator: Arc<Coordinator<'static, _, _, _, _, _>>| {
                ws.on_upgrade(|websocket| -> Pin<Box<dyn Future<Output = ()> + Send>> {
                    graphql_subscriptions(websocket, coordinator, ctx).boxed()
                })
            },
        ))
    .map(|reply| {
        // TODO#584: remove this workaround
        warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws")
    })
    .or(warp::post()
        .and(warp::path("graphql"))
        .and(qm_graphql_filter))
    .or(warp::get()
        .and(warp::path("playground"))
        .and(playground_filter("/graphql", Some("/subscriptions"))))
    .or(homepage)
    .with(log);

    warp::serve(routes).run(([127, 0, 0, 1], 8888)).await;
}
