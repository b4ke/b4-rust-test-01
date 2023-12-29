//auth: mic
//ver: 0.0.1
//time: 2023-12-28 22:00:00;
use axum::{
    routing::get,
    Router,
};
use hyper::Server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let addr = SocketAddr::new([0, 0, 0, 0].into(), 3000);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
