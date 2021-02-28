use juniper::{graphql_object, EmptyMutation, EmptySubscription, GraphQLObject, RootNode};

use crate::result::Result;

#[derive(Clone)]
pub struct Context {}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    pub fn photos() -> Result<Vec<Photo>> {
        let photos = vec![
            Photo { url: "https://lh3.googleusercontent.com/rsWOt-AVz68ChCYrpVVo1NXqqJzOn312YaI-oGCwXhHgROdPhnD42HBgW5AYwvf0OhjKYHgM-UafS6II3VfVWu5lkuZuTv0j1gHNYmGoCyhxo8cw9ecloQNdAERuQ3aBIOkG9CpiVH0l5rCge6tMXS45ms4TG5mha9IKRx6Bqy9GNCVG3fpfSTjbHSXey3DUj8bhf0PbdDO0zoDa0-OVMWDIt19xqF9Y6b1N4aLZAmh4GICmU42Gsqs6Dg202rIbbm9cZxEE4AYLhRr0We5jBxnBd37-Y6-hyB6UoVpz7Ds46dcK50HJmWsrqT4Bv9hw0dDSvjFMJnd6BEFNpNY4rrXPprRHRS4Z8dcJlLcKOjG_vy1njViWzAY2RBPaId-5ZYVYyHNQrAJ7KwmrWkkFc8GCKp2SIoQWbIzL3awyISYRIu2NZsAh6FwGJ4yNNQuXXeNXaJ-qXY4a1_cr4eu4hPeqmCTrzTDI4_I4rEpj1tmH-urh5tIFxI_50t6aS6Rv30zOqP65TB7mYWGIh0lDM2hOBYBq141lltzJ57ASMZvVmsLLr9Cdn2Qs7i-ukvV36D1uGMdWLuzLeldUoLPtEBpfMeEFaa4LGUcTruNCAcLb-FYjztK4cC7NAQUVrlalBczNPSJnTwi-0awTXH1Yc3Bn-md0jvftlhkQMp4UdeEX91bSMfElzYBBqaxzKo0=w1146-h2036-no".into(),
            }
        , Photo { 
            url: "https://lh3.googleusercontent.com/n_AAi1Lv97ULSKdRFENCYuDirHaOvDxKPfGCQ7KfBsC4PFXXNrNP9JMNJVTpEvbvqyGmuGSGOnR-zFMN6TSsHwzYppjoOPIZTuLs7OgzlwPWTd0Ws_ylEfQDRp9aaqX6bGEhqN_rqYWe5CKzF8h9THx24umCQhytXyx0S-cBDRq59L9yg1jvZ3HCLmIHP9zjxiLJVofY_ggMzz12uxuwIysTuhiOqczU_cp_stXQB_rzGNN8dC66RAemfqoZ8WPtLr8_pVGgfmppF4hh9Z9n5lpgi-CQP91nOMr80MNYcd1UZ-uQEwgcynmU1e5E4zsyXYtMaeS38r-iOdQozVqR6uxuHAkaRUifcN5nGccgJBrXTazTsOXJSerhYiXDe5luqpNvFw_ipoLbCp7zOOEP6xnhB7lZ0VIGmYXmYjhjGTeytwl842OZv-tB95X_xRE5F6ijt366aVwQEyWcA15M7tGb5QkRyst1jgp90eq24E6WjXQYWaNCFvlHDOPD2mPIF-sRJNXP6X9tKrKRVeaLXg9QNx8ZYNs_Z1HR17XfqWkRVQCSdQsmfSQEmgyyAPNweIGZ9d1ZdfZGAO2hlmwwx3e8QBNQsSHgV-JEexl6ktcW5VLHOZvMff34LZtlXyiU3-I6y0dYQLUpaftIoI_Ubxo3nO8qPxOpoOiCWDa-cwC_UkzTfPaHVGKQMVS4tQM=w1528-h2036-no".into()
        }
        ];
        Ok(photos)
    }
}

// Photos

#[derive(GraphQLObject)]
struct Photo {
    url: String,
}
