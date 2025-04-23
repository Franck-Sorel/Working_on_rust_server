use axum::{
    body::Body,
    response::Response,
    routing::{get, post},
    Router,
};
use hyper::StatusCode;
// use std::net::SocketAddr;
use tokio::net::TcpListener;

// asynchrnuous funtion environement for Rust..
#[tokio::main]
async fn main() {
    // let addr = Router::new().route("/", get(response)).route("/path", get(|| async { "Second method has been reached"}));
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/path", post("br"));
    
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind("localhost:7878").await.unwrap();

    println!("Listening on http://localhost:7878");

    axum::serve(listener, app).await.unwrap();
}

async  fn response() -> Response {
    Response::builder().status(StatusCode::CREATED).header("key", "value").body(Body::from("string")).unwrap()
}

async fn hello_world() -> &'static str {
    "hello world"
}