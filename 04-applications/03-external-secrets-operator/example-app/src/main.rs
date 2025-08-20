use axum::{
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use tokio::time::{sleep, Duration};

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    timestamp: String,
    secrets_loaded: bool,
    environment: String,
}

#[derive(Serialize)]
struct SecretsResponse {
    postgres_host: Option<String>,
    postgres_username: Option<String>,
    postgres_database: Option<String>,
    postgres_password_length: Option<usize>,
    all_env_vars: HashMap<String, String>,
}

#[derive(Serialize, Clone)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
    secrets_status: String,
}

static mut STARTUP_LOGS: Vec<LogEntry> = Vec::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{}", "🚀 Starting Banana Back Application...".green().bold());

    // Attendre un peu pour que les secrets soient disponibles
    sleep(Duration::from_secs(2)).await;

    log_message("INFO", "Application starting up");

    // Lire et afficher les secrets au démarrage
    display_secrets_info().await;

    // Démarrer le serveur web
    start_web_server().await?;

    Ok(())
}

async fn display_secrets_info() {
    println!("{}", "📋 Reading secrets from environment variables...".blue().bold());
    println!("{}", "=" .repeat(60).blue());

    // Lire les variables PostgreSQL
    let postgres_host = env::var("POSTGRES_HOST");
    let postgres_username = env::var("POSTGRES_USERNAME");
    let postgres_database = env::var("POSTGRES_DATABASE");
    let postgres_password = env::var("POSTGRES_PASSWORD");

    // Afficher les informations de connexion PostgreSQL
    println!("{}", "🐘 PostgreSQL Configuration:".cyan().bold());

    match postgres_host {
        Ok(host) => {
            println!("  {} {}", "✅ HOST:".green(), host);
            log_message("INFO", &format!("PostgreSQL host loaded: {}", host));
        }
        Err(_) => {
            println!("  {} Not found", "❌ HOST:".red());
            log_message("ERROR", "PostgreSQL host not found in environment");
        }
    }

    match postgres_username {
        Ok(username) => {
            println!("  {} {}", "✅ USERNAME:".green(), username);
            log_message("INFO", &format!("PostgreSQL username loaded: {}", username));
        }
        Err(_) => {
            println!("  {} Not found", "❌ USERNAME:".red());
            log_message("ERROR", "PostgreSQL username not found in environment");
        }
    }

    match postgres_database {
        Ok(database) => {
            println!("  {} {}", "✅ DATABASE:".green(), database);
            log_message("INFO", &format!("PostgreSQL database loaded: {}", database));
        }
        Err(_) => {
            println!("  {} Not found", "❌ DATABASE:".red());
            log_message("ERROR", "PostgreSQL database not found in environment");
        }
    }

    match postgres_password {
        Ok(password) => {
            let masked_password = "*".repeat(password.len());
            println!("  {} {} (length: {})", "✅ PASSWORD:".green(), masked_password, password.len());
            log_message("INFO", &format!("PostgreSQL password loaded (length: {})", password.len()));
        }
        Err(_) => {
            println!("  {} Not found", "❌ PASSWORD:".red());
            log_message("ERROR", "PostgreSQL password not found in environment");
        }
    }

    println!("{}", "=" .repeat(60).blue());

    // Afficher toutes les variables d'environnement (filtrées)
    println!("{}", "🔧 Environment Variables:".cyan().bold());
    let mut env_vars: Vec<(String, String)> = env::vars().collect();
    env_vars.sort_by(|a, b| a.0.cmp(&b.0));

    for (key, value) in env_vars {
        if key.starts_with("POSTGRES_") ||
            key == "NODE_ENV" ||
            key == "PORT" ||
            key.starts_with("K8S_") ||
            key.starts_with("SCW_") {
            if key.contains("PASSWORD") || key.contains("SECRET") || key.contains("TOKEN") {
                let masked_value = "*".repeat(value.len());
                println!("  {} = {} (length: {})", key.yellow(), masked_value, value.len());
            } else {
                println!("  {} = {}", key.yellow(), value.green());
            }
        }
    }

    println!("{}", "=" .repeat(60).blue());

    // Test de connexion simulé
    simulate_database_connection().await;
}

async fn simulate_database_connection() {
    println!("{}", "🔌 Simulating database connection...".blue());

    let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let username = env::var("POSTGRES_USERNAME").unwrap_or_else(|_| "unknown".to_string());
    let database = env::var("POSTGRES_DATABASE").unwrap_or_else(|_| "unknown".to_string());
    let password_exists = env::var("POSTGRES_PASSWORD").is_ok();

    if password_exists {
        println!("  {} Attempting connection to postgresql://{}@{}/{}", "🔄".blue(), username, host, database);
        sleep(Duration::from_millis(500)).await;
        println!("  {} Connection successful! (simulated)", "✅".green());
        log_message("INFO", "Database connection test successful (simulated)");
    } else {
        println!("  {} Cannot connect: missing password", "❌".red());
        log_message("ERROR", "Database connection failed: missing password");
    }
}

async fn start_web_server() -> anyhow::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    println!("{}", format!("🌐 Starting web server on port {}...", port).green());
    log_message("INFO", &format!("Web server starting on port {}", port));

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/ready", get(ready_handler))
        .route("/secrets", get(secrets_handler))
        .route("/logs", get(logs_handler))
        .route("/env", get(env_handler));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    println!("{}", "✅ Server ready! Available endpoints:".green().bold());
    println!("  {} GET  /        - Welcome message", "📍".blue());
    println!("  {} GET  /health  - Health check", "📍".blue());
    println!("  {} GET  /ready   - Readiness check", "📍".blue());
    println!("  {} GET  /secrets - Secrets information", "📍".blue());
    println!("  {} GET  /logs    - Application logs", "📍".blue());
    println!("  {} GET  /env     - Environment variables", "📍".blue());

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "🍌 Welcome to Banana Back API!",
        "version": "1.0.0",
        "endpoints": [
            "/health - Health check",
            "/ready - Readiness check", 
            "/secrets - Secrets information",
            "/logs - Application logs",
            "/env - Environment variables"
        ]
    }))
}

async fn health_handler() -> Json<HealthResponse> {
    let secrets_loaded = env::var("POSTGRES_PASSWORD").is_ok();

    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        secrets_loaded,
        environment: env::var("NODE_ENV").unwrap_or_else(|_| "development".to_string()),
    })
}

async fn ready_handler() -> Result<Json<serde_json::Value>, StatusCode> {
    // Vérifier que tous les secrets requis sont présents
    let required_secrets = ["POSTGRES_HOST", "POSTGRES_USERNAME", "POSTGRES_DATABASE", "POSTGRES_PASSWORD"];
    let mut missing_secrets = Vec::new();

    for secret in required_secrets {
        if env::var(secret).is_err() {
            missing_secrets.push(secret);
        }
    }

    if missing_secrets.is_empty() {
        Ok(Json(serde_json::json!({
            "status": "ready",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "message": "All required secrets are loaded"
        })))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn secrets_handler() -> Json<SecretsResponse> {
    let postgres_host = env::var("POSTGRES_HOST").ok();
    let postgres_username = env::var("POSTGRES_USERNAME").ok();
    let postgres_database = env::var("POSTGRES_DATABASE").ok();
    let postgres_password_length = env::var("POSTGRES_PASSWORD").ok().map(|p| p.len());

    // Collecter toutes les variables d'environnement (sans les valeurs sensibles)
    let mut all_env_vars = HashMap::new();
    for (key, value) in env::vars() {
        if key.contains("PASSWORD") || key.contains("SECRET") || key.contains("TOKEN") {
            all_env_vars.insert(key, format!("****** (length: {})", value.len()));
        } else {
            all_env_vars.insert(key, value);
        }
    }

    Json(SecretsResponse {
        postgres_host,
        postgres_username,
        postgres_database,
        postgres_password_length,
        all_env_vars,
    })
}

async fn logs_handler() -> Json<Vec<LogEntry>> {
    unsafe { Json(STARTUP_LOGS.clone()) }
}

async fn env_handler() -> Json<HashMap<String, String>> {
    let mut filtered_env = HashMap::new();

    for (key, value) in env::vars() {
        if key.starts_with("POSTGRES_") ||
            key == "NODE_ENV" ||
            key == "PORT" ||
            key.starts_with("K8S_") {
            if key.contains("PASSWORD") || key.contains("SECRET") || key.contains("TOKEN") {
                filtered_env.insert(key, format!("****** (length: {})", value.len()));
            } else {
                filtered_env.insert(key, value);
            }
        }
    }

    Json(filtered_env)
}

fn log_message(level: &str, message: &str) {
    let log_entry = LogEntry {
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: level.to_string(),
        message: message.to_string(),
        secrets_status: if env::var("POSTGRES_PASSWORD").is_ok() {
            "loaded".to_string()
        } else {
            "missing".to_string()
        },
    };

    unsafe {
        STARTUP_LOGS.push(log_entry);
    }
}