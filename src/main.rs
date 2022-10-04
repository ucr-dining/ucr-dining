// This example requires the following input to succeed:
// { "command": "do something" }

use lambda_http::{http::Method, tower::ServiceBuilder, Body, Request, Response};
use lambda_runtime::{service_fn, Error};
use reqwest::StatusCode;
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};

/// This is also a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
// #[derive(Deserialize)]
// struct Request {
//     command: String,
// }

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
// #[derive(Serialize)]
// struct Response {
//     lothian: String,
//     glasgow: String,
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let cors_layer = CorsLayer::new()
        .allow_methods(vec![Method::GET])
        .allow_origin(Any);

    let handler = ServiceBuilder::new()
        .layer(cors_layer)
        .service(service_fn(my_handler));

    lambda_http::run(handler).await?;
    Ok(())
}

#[derive(Debug, Serialize)]
struct Out {
    lothian: String,
    glasgow: String,
}

pub(crate) async fn my_handler(_event: Request) -> Result<Response<Body>, Error> {
    // client code from https://docs.rs/awc/latest/awc/ & discovered on actix github
    let client = reqwest::Client::new();

    let mut body = Out {
        lothian: String::new(),
        glasgow: String::new(),
    };

    for (i, name) in ["https://foodpro.ucr.edu/foodpro/shortmenu.asp?sName=University+of+California%2C+Riverside+Dining+Services&locationNum=02&locationName=Lothian+Residential+Restaurant&naFlag=1", "https://foodpro.ucr.edu/foodpro/shortmenu.asp?sName=University%20of%20California%2C%20Riverside%20Dining%20Services&locationNum=03&locationName=Glasgow&naFlag=1"].into_iter().enumerate() {
        let req = client.get(name);
        // make request to actual server now
        let res = req.send().await?;

        if res.status() != StatusCode::from_u16(200).unwrap() {
            return Ok(lambda_http::Response::builder().status(res.status()).body(Body::from(res.text().await?))?);
        } else {
            let bytes = &res.bytes().await?;
            let text = std::str::from_utf8(bytes).unwrap();
            match i {
                0 => {
                    body.lothian.push_str(text);
                }
                1 => {
                    body.glasgow.push_str(text);
                }
                _ => {unreachable!()}
            }
        }
    }

    Ok(lambda_http::Response::builder()
        .status(200)
        .body(Body::from(serde_json::to_string(&body)?))?)
    // Ok(Response::builder()
    //     .status(200)
    //     .body(Body::from("hello world!"))?)
}
