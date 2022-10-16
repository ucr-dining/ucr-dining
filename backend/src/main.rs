use lambda_http::{http::Method, tower::ServiceBuilder, Body, Request, Response};
use lambda_runtime::{service_fn, Error};
use reqwest::StatusCode;
use tower_http::cors::{Any, CorsLayer};
use ucr_dining_lambda::{OutRes, parse, HALL_REQUESTS, HallType};

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

pub(crate) async fn my_handler(_event: Request) -> Result<Response<Body>, Error> {
    // client code from https://docs.rs/awc/latest/awc/ & discovered on actix github
    let client = reqwest::Client::new();

    let mut body = OutRes::default();

    for hall in HALL_REQUESTS.iter() {
        let req = client.get(hall.url);
        // make request to actual server now
        let res = req.send().await?;

        if res.status() != StatusCode::from_u16(200).unwrap() {
            return Ok(lambda_http::Response::builder().status(res.status()).body(Body::from(res.text().await?))?);
        } else {
            let bytes = &res.bytes().await?;
            let text = std::str::from_utf8(bytes).unwrap();
            
            let dining_hall = parse(text, hall.hall);

            // feel like the below code is redundant
            match hall.hall {
                HallType::Lothian => {
                    body.lothian = dining_hall;
                }
                HallType::Glasgow => {
                    body.glasgow = dining_hall;
                }
            }
        }
    }

    Ok(lambda_http::Response::builder()
        .status(200)
        .body(Body::from(serde_json::to_string(&body)?))?)
}
