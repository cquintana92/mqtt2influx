use actix_web::{web, App, HttpResponse, HttpServer};
use mqtt2influx_core::anyhow::Result;
use std::sync::Arc;

mod request_id_middleware;
mod request_logger_middleware;
mod types;

pub use types::*;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct ApiValuesResponse {
    values: Vec<ApiEvent>,
}

async fn get(state: web::Data<Arc<ApiState>>) -> HttpResponse {
    let values = state.values().await;
    HttpResponse::Ok().json(&ApiValuesResponse { values })
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().body("Running")
}

pub async fn run(port: u16, state: Arc<ApiState>) -> Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Started API [http://{}]", &addr);
    HttpServer::new(move || {
        let ignored = vec!["/health".to_string()];
        App::new()
            .wrap(request_logger_middleware::RequestLogger::new_with_ignored_paths(ignored))
            .wrap(request_id_middleware::RequestId::default())
            .data(state.clone())
            .route("/", web::get().to(get))
            .route("/health", web::get().to(health))
    })
    .bind(addr)?
    .run()
    .await?;
    Ok(())
}
