mod benchmarks;
mod errors;

use crate::errors::AppError;

use axum::{
    routing::get,
    Router,
};

use std::env;
use std::path::PathBuf;

use tower_http::trace::TraceLayer;

use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

#[tokio::main]
async fn main() {
    // -------------------------------
    // Initialize Tracing / Logging
    // -------------------------------
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("SoroScope Starting...");

    // -------------------------------
    // CLI Argument Handling (Benchmark)
    // -------------------------------
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "benchmark" {
        tracing::info!("Starting SoroScope Benchmark...");

        let possible_paths = vec![
            "target/wasm32-unknown-unknown/release/soroban_token_contract.wasm",
            "../target/wasm32-unknown-unknown/release/soroban_token_contract.wasm",
        ];

        let mut wasm_path = None;

        for p in possible_paths {
            let path = PathBuf::from(p);
            if path.exists() {
                wasm_path = Some(path);
                break;
            }
        }

        if let Some(path) = wasm_path {
            if let Err(e) = benchmarks::run_token_benchmark(path) {
                tracing::error!("Benchmark failed: {}", e);
            }
        } else {
            tracing::error!(
                "Could not find soroban_token_contract.wasm. Build the contract first."
            );
        }

        return;
    }

    // -------------------------------
    // Web Server Setup
    // -------------------------------
    tracing::info!("Starting SoroScope API Server...");

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_check))
        .route(
            "/error",
            get(|| async {
                Err::<&str, AppError>(AppError::BadRequest(
                    "Test error".to_string(),
                ))
            }),
        )
        .layer(TraceLayer::new_for_http());

    // -------------------------------
    // Run Server
    // -------------------------------
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server listening on http://0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

// -------------------------------
// Handlers
// -------------------------------

async fn health_check() -> &'static str {
    "OK"
}

async fn root_handler() -> &'static str {
    "Hello from SoroScope! Usage: cargo run -p soroscope-core -- benchmark"
}
