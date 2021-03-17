use std::env;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_new;

use std::convert::Infallible;

use bytes::Buf;
use dotenv::dotenv;
use futures::stream::TryStreamExt;
use futures::Stream;
use mime::Mime;
use mpart_async::server::MultipartStream;
use serde_derive::Serialize;
use sqlx::sqlite::SqlitePool;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use warp::{http::StatusCode, Filter};
use warp::{Rejection, Reply};

mod config;
mod db;
mod entities;
mod graphql;
mod juniper_warp;
mod repo;
mod result;
mod use_cases;

#[cfg(test)]
mod integration_tests;

use graphql::{schema, Context};

lazy_static! {
    pub static ref POOL: SqlitePool = db::setup();
    static ref DOMAIN: String = "localhost".into();
}

#[tokio::main] // or #[tokio::main]
async fn main() {
    dotenv().ok();
    config::init();

    env::set_var("RUST_LOG", "info");

    // Initialize resources
    env_logger::init();
    result::init_error_tracking();

    db::migrate().await;

    let routes = get_routes();

    log::info!("Listening on 127.0.0.1:8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
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
    let qm_schema = schema();
    let qm_state = warp::any().map(move || Context::new());
    let qm_graphql_filter = juniper_warp::make_graphql_filter(qm_schema, qm_state.boxed());

    let spa_files = warp::get().and(warp::fs::file("assets/public/index.html"));
    let files_dir = config::files_dir();
    let public_files = warp::path("public").and(warp::fs::dir(files_dir));
    let main_js = warp::path("main.js").and(warp::fs::file("assets/public/main.js"));
    let dist_assets = warp::path("dist").and(warp::fs::dir("assets/public/dist"));

    let graphql_route = warp::post()
        .and(warp::path("graphql"))
        .and(qm_graphql_filter);

    let register_route = warp::post()
        .and(warp::path("register"))
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .and(qm_state)
        .and_then(
            |body: use_cases::RegisterRequest, context: Context| async move {
                context
                    .user
                    .register(&body)
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
            },
        );

    // Upload route
    let upload = warp::path!("upload")
        .and(warp::post())
        .and(warp::header::<Mime>("content-type"))
        .and(warp::body::stream())
        .and(qm_state)
        .and_then(mpart);

    graphql_route
        .or(register_route)
        .or(upload)
        .or(main_js)
        .or(dist_assets)
        .or(public_files)
        .or(spa_files)
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

async fn mpart(
    mime: Mime,
    body: impl Stream<Item = Result<impl Buf, warp::Error>> + Unpin,
    ctx: Context,
) -> Result<impl warp::Reply, Infallible> {
    let boundary = mime.get_param("boundary").map(|v| v.to_string()).unwrap();

    let mut stream = MultipartStream::new(
        boundary,
        body.map_ok(|mut buf| buf.copy_to_bytes(buf.remaining())),
    );

    while let Ok(Some(mut field)) = stream.try_next().await {
        println!("Field received:{}", field.name().unwrap());

        let id = Uuid::new_v4();
        let filename = format!("{}.kind", id);

        let mut file = File::create(config::file_path(&filename)).await.unwrap();

        while let Ok(Some(bytes)) = field.try_next().await {
            println!("Bytes received:{}", bytes.len());
            file.write_all(&bytes).await.unwrap();
        }
        let kind = infer::get_from_path(config::file_path(&filename))
            .expect("file read successfully")
            .expect("file type is known");
        let filename_with_kind = format!("{}.{}", id, kind.extension());

        tokio::fs::rename(
            config::file_path(&filename),
            config::file_path(&filename_with_kind),
        )
        .await
        .unwrap();

        ctx.photo.create(&filename_with_kind).await.unwrap();
    }

    Ok(format!("Thanks!\n"))
}
