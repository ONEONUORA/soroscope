use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig {
    server_port: u16,
    rust_log: String,
    soroban_rpc_url: String,
}

fn load_config() -> Result<AppConfig, ConfigError> {
    // Load .env file if present
    dotenvy::dotenv().ok();

    let settings = Config::builder()
        .add_source(config::Environment::default())
        .set_default("server_port", 3000)?
        .set_default("rust_log", "info")?
        .set_default("soroban_rpc_url", "https://soroban-testnet.stellar.org")?
        .build()?;

    settings.try_deserialize()
}

fn main() {
    let config = load_config().expect("Failed to load configuration");
    println!("SoroScope CLI Initialized with config: {:?}", config);
}
