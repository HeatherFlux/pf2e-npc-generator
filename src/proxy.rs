use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use actix_web::middleware::Logger;

async fn proxy_ollama(body: web::Json<serde_json::Value>) -> HttpResponse {
    let client = reqwest::Client::new();
    match client
        .post("http://localhost:11434/api/generate")
        .json(&body)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(json) => HttpResponse::Ok().json(json),
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .route("/api/generate", web::post().to(proxy_ollama))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
} 