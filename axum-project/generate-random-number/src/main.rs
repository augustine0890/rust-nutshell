use axum::{extract::Query, response::Html, routing::get, Router};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// `Deserialize` need be implemented to use with `Query` extractor
#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}
/*
Run the program: http://localhost:3000/?start=1&end=10
 */
async fn handler(Query(range): Query<RangeParameters>) -> Html<String> {
    let random_number = thread_rng().gen_range(range.start..range.end);

    // Send response in html format
    Html(format!("<h1>Random Number: {}</h1>", random_number))
}
