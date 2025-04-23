use axum::{
    body::Body, extract::Multipart, http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Router
};
use tokio::{fs, net::TcpListener};
#[tokio::main]
async fn main() {
    let app = Router::new().route("/upload", post(post_handler)).route("/", get(print));

    let listener = TcpListener::bind("localhost:7878").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn post_handler(mut multipart: Multipart) -> Response {
    while let Some(field) = multipart
        .next_field().await
        
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response()).expect("msg")
    {
        if let Some("file") = field.name() {            
            println!("{:?}, {:?}", field.file_name(), field.content_type());
            // let mut des = field.file_name().expect("msg");
            // let des = format!("/upload/{}", des);
            // let content = fs::read_to_string(&des).await.expect("msg");
            
        }
    }

    Response::builder().status(StatusCode::CREATED).header("key", "value").body(Body::from(r#"{"reponse": "response_body from the server"}"#)).unwrap()

    // Ok(StatusCode::OK.into_response(),)

}

async  fn print() -> &'static str { 
    "Hello world"
}

async fn post_2() -> impl IntoResponse {
    (StatusCode::CREATED,"reponse_to the post request")
}