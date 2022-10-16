use lambda_http::{http::Method, tower::ServiceBuilder, Body, Request, Response};
use lambda_runtime::{service_fn, Error};
use tower_http::cors::{Any, CorsLayer};

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
    Ok(lambda_http::Response::builder()
        .status(200)
        .body(Body::from(String::from("Hello World!")))?)
}
