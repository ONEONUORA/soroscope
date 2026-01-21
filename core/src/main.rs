use axum::{
    extract::Json,
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Deserialize, ToSchema)]
struct AnalyzeRequest {
    #[schema(example = "0x1234...")]
    contract_id: String,
    #[schema(example = "invoke")]
    function_name: String,
}

#[derive(Serialize, ToSchema)]
struct ResourceReport {
    #[schema(example = 1000)]
    cpu_instructions: u64,
    #[schema(example = 2048)]
    memory_bytes: u64,
    #[schema(example = 512)]
    ledger_read_bytes: u64,
    #[schema(example = 256)]
    ledger_write_bytes: u64,
}

#[utoipa::path(
    post,
    path = "/analyze",
    request_body = AnalyzeRequest,
    responses(
        (status = 200, description = "Resource analysis successful", body = ResourceReport),
        (status = 500, description = "Analysis failed")
    ),
    tag = "Analysis"
)]
async fn analyze(Json(payload): Json<AnalyzeRequest>) -> Result<Json<ResourceReport>, StatusCode> {
    // Placeholder implementation
    let report = ResourceReport {
        cpu_instructions: 1000,
        memory_bytes: 2048,
        ledger_read_bytes: 512,
        ledger_write_bytes: 256,
    };
    Ok(Json(report))
}

#[derive(OpenApi)]
#[openapi(
    paths(analyze),
    components(schemas(AnalyzeRequest, ResourceReport)),
    tags(
        (name = "Analysis", description = "Soroban contract resource analysis endpoints")
    ),
    info(
        title = "SoroScope API",
        version = "0.1.0",
        description = "API for analyzing Soroban smart contract resource consumption"
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/analyze", post(analyze))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("SoroScope API running on http://localhost:8080");
    println!("Swagger UI available at http://localhost:8080/swagger-ui");

    axum::serve(listener, app).await.unwrap();
}
