use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use shared::{Message, UserInfo};
use std::collections::HashMap;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, Level};

#[derive(Deserialize)]
struct QueryParams {
    name: Option<String>,
    limit: Option<u32>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("🚀 Starting Rust web server...");

    // Build our application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user))
        .route("/search", get(search))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        );

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    info!("🌐 Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "🦀 Welcome to the Rust Web Server! Try /health, /users/123, or /search?name=rust"
}

async fn health_check() -> Json<Message> {
    Json(
        Message::new("🦀 Rust web server is running!")
            .with_timestamp(chrono::Utc::now().to_rfc3339())
            .with_metadata("status".to_string(), "healthy".to_string()),
    )
}

async fn get_user(Path(user_id): Path<u32>) -> Result<Json<UserInfo>, StatusCode> {
    // In a real app, you'd fetch from a database
    match user_id {
        1 => Ok(Json(
            UserInfo::new("Alice").with_email("alice@example.com".to_string()),
        )),
        2 => Ok(Json(
            UserInfo::new("Bob").with_email("bob@example.com".to_string()),
        )),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_user(Json(user): Json<UserInfo>) -> Result<Json<UserInfo>, StatusCode> {
    // In a real app, you'd save to a database
    info!("Creating user: {:?}", user.name);
    Ok(Json(user))
}

async fn search(Query(params): Query<QueryParams>) -> Json<serde_json::Value> {
    let mut response = HashMap::new();
    response.insert("query", params.name.unwrap_or_else(|| "none".to_string()));
    response.insert("limit", params.limit.unwrap_or(10).to_string());
    response.insert("results", "This would contain search results".to_string());

    Json(serde_json::to_value(response).unwrap())
}
