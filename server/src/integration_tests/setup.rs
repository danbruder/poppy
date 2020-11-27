use std::env;

use serde_json::{self, Value as Json};
use string_template::Template;

use crate::db;

#[derive(Debug)]
pub struct TestResponse {
    pub status_code: i32,
    pub body: Option<String>,
    pub cookie: Option<String>,
    pub content_type: String,
}

pub struct Client {}

impl Client {
    pub fn new() -> Self {
        Client {}
    }

    pub async fn post(&self, route: &str, body: Json) -> TestResponse {
        env::set_var("DATABASE_URL", "sqlite::memory:");
        db::migrate().await;
        let filter = crate::get_routes();

        let response = warp::test::request()
            .method("POST")
            .header("content-type", "application/json")
            .path(route)
            .body(serde_json::to_string(&body).expect("Could not serialize json"))
            .reply(&filter)
            .await;

        TestResponse {
            status_code: response.status().as_u16() as i32,
            body: Some(String::from_utf8(response.body().to_owned().to_vec()).unwrap()),
            cookie: response.headers().get("set-cookie").map(|i| {
                i.to_str()
                    .expect("could not parse cookie header")
                    .to_owned()
            }),
            content_type: response
                .headers()
                .get("content-type")
                .expect("missing content-type header in warp response")
                .to_str()
                .expect("invalid content-type string")
                .to_owned(),
        }
    }

    pub async fn post_graphql(&self, body: &str) -> TestResponse {
        env::set_var("DATABASE_URL", "sqlite::memory:");
        db::migrate().await;

        let filter = crate::get_routes();

        let response = warp::test::request()
            .method("POST")
            .header("content-type", "application/json")
            .path("/graphql")
            .body(body)
            .reply(&filter)
            .await;

        TestResponse {
            status_code: response.status().as_u16() as i32,
            body: Some(String::from_utf8(response.body().to_owned().to_vec()).unwrap()),
            cookie: None,
            content_type: response
                .headers()
                .get("content-type")
                .expect("missing content-type header in warp response")
                .to_str()
                .expect("invalid content-type string")
                .to_owned(),
        }
    }

    pub async fn query(&self, query: &str) -> Json {
        let full_query = format!(
            "{}{}{}",
            r#"{"query": "#,
            snailquote::escape(&query),
            r#"}"#
        );

        let response = self.post_graphql(&full_query).await;

        if response.status_code != 200 {
            println!("Query = {}", &query);
            println!("Status code = {}", &response.status_code);
            println!("Response = {:?}", &response);
        }

        assert_eq!(response.status_code, 200);
        assert_eq!(response.content_type, "application/json");
        unwrap_json_response(&response)
    }

    pub async fn query_with_args(&self, query: &str, vars: &[&str]) -> Json {
        let query_and_vars = Template::new(query).render_positional(vars);
        self.query(&query_and_vars).await
    }
}

pub fn unwrap_json_response(response: &TestResponse) -> Json {
    serde_json::from_str::<Json>(
        response
            .body
            .as_ref()
            .expect("No data returned from request"),
    )
    .expect("Could not parse JSON object")
}

impl Drop for Client {
    fn drop(&mut self) {
        //cleanup_test_pool(&self.db_name)
    }
}
