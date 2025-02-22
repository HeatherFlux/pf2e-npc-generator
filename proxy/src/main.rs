use axum::{
    routing::post,
    Router,
    Json,
    http::StatusCode,
};
use tower_http::cors::CorsLayer;
use serde_json::Value;
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn proxy_request(Json(body): Json<Value>) -> Result<Json<Value>, StatusCode> {
    let client = reqwest::Client::new();
    
    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&body)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    if !response.status().is_success() {
        return Err(StatusCode::BAD_GATEWAY);
    }

    let json = response
        .json::<Value>()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json))
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/api/generate", post(proxy_request))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Proxy server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
} 