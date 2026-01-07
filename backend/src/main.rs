mod handlers;
mod models;

use axum::{
    Router,
    http::{StatusCode, Uri, header, HeaderMap, HeaderValue},
    response::{IntoResponse, Response},
    routing::{get, post, put, delete},
};
use mongodb::{Client, Database};
use rust_embed::{Embed, RustEmbed};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use handlers::todos;

// Embed the frontend dist folder into the binary
#[derive(RustEmbed)]
#[folder = "../frontend/dist/"]
struct Asset;

// Application state shared across handlers
pub struct AppState {
    pub db: Database,
}

#[tokio::main]
async fn main() {
    // Initialize tracing with a default level if RUST_LOG is not set
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug"));

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    // Log embedded files at startup for debugging
    tracing::info!("Files embedded in binary:");
    for file in Asset::iter() {
        tracing::info!("  - {}", file);
    }

    // Load environment variables
    dotenvy::dotenv().ok();

    // Get MongoDB URI from environment
    let mongodb_uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    tracing::info!("Connecting to MongoDB at {}...", mongodb_uri);

    // Connect to MongoDB
    let client_options = mongodb::options::ClientOptions::parse(&mongodb_uri)
        .await
        .expect("Failed to parse MongoDB URI");
    
    let client = Client::with_options(client_options)
        .expect("Failed to create MongoDB client");

    let db = client.database("todos");

    tracing::info!("Connected to MongoDB");

    // Create shared state
    let state = Arc::new(AppState { db });

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build API routes
    let api_routes = Router::new()
        .route("/todos", get(todos::list_todos))
        .route("/todos", post(todos::create_todo))
        .route("/todos/{id}", get(todos::get_todo))
        .route("/todos/{id}", put(todos::update_todo))
        .route("/todos/{id}", delete(todos::delete_todo));

    // Build the main application router
    let app = Router::new()
        .nest("/api", api_routes)
        .fallback(static_handler)
        .layer(cors)
        .with_state(state);

    // Get port from environment or default to 8080
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

/// Serves static files from the embedded frontend dist folder.
/// Falls back to index.html for SPA routing.
/// Supports serving pre-compressed Brotli (.br) and Gzip (.gz) assets.
async fn static_handler(uri: Uri, headers: HeaderMap) -> Response {
    let mut path = uri.path().trim_start_matches('/').to_string();
    
    // 1. Handle root or empty path
    if path.is_empty() || path == "index.html" {
        path = "index.html".to_string();
    }

    // Get the preferred encoding from the client
    let accept_encoding = headers
        .get(header::ACCEPT_ENCODING)
        .and_then(|v: &HeaderValue| v.to_str().ok())
        .unwrap_or("");

    let supports_br = accept_encoding.contains("br");
    let supports_gzip = accept_encoding.contains("gzip");

    // Helper to serve a file with optional compression
    async fn serve_asset(path: &str, supports_br: bool, supports_gzip: bool) -> Option<Response> {
        let mime = mime_guess::from_path(path).first_or_octet_stream();

        // 1. Try Brotli
        if supports_br {
            let br_path = format!("{}.br", path);
            if let Some(content) = Asset::get(&br_path) {
                return Some((
                    [
                        (header::CONTENT_TYPE, mime.as_ref()),
                        (header::CONTENT_ENCODING, "br"),
                    ],
                    content.data.into_owned(),
                ).into_response());
            }
        }

        // 2. Try Gzip
        if supports_gzip {
            let gz_path = format!("{}.gz", path);
            if let Some(content) = Asset::get(&gz_path) {
                return Some((
                    [
                        (header::CONTENT_TYPE, mime.as_ref()),
                        (header::CONTENT_ENCODING, "gzip"),
                    ],
                    content.data.into_owned(),
                ).into_response());
            }
        }

        // 3. Fallback to uncompressed
        if let Some(content) = Asset::get(path) {
            return Some((
                [(header::CONTENT_TYPE, mime.as_ref())],
                content.data.into_owned(),
            ).into_response());
        }

        None
    }

    // 2. Try exact match for assets
    if let Some(response) = serve_asset(&path, supports_br, supports_gzip).await {
        return response;
    }

    // 3. SPA Fallback: Serve index.html for non-file requests (routes)
    // Avoid falling back if the path contains a dot (likely a missing asset)
    if !path.contains('.') {
        tracing::debug!("Route '{}' not found, falling back to index.html", path);
        if let Some(response) = serve_asset("index.html", supports_br, supports_gzip).await {
            return response;
        }
    }

    // List of "quiet" files that some browsers or extensions probe for
    let quiet_files = ["sw.js", "favicon.ico", "manifest.json"];
    let is_well_known = path.contains(".well-known");

    if quiet_files.contains(&path.as_str()) || is_well_known {
        tracing::debug!("Quietly ignoring missing system/probe file: '{}'", path);
    } else {
        tracing::error!("File not found: '{}'", path);
    }

    (StatusCode::NOT_FOUND, format!("File not found: {}", path)).into_response()
}
