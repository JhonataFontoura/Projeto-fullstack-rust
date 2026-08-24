use axum::{extract::{Path, State}, http::StatusCode, response::{Html, IntoResponse, Redirect}, routing::{get, post}, Form, Json, Router};
use askama::Template;
use rust_decimal::Decimal;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod config;
mod models;
mod services;

use models::{Asset, AssetForm, AssetView};

#[derive(Clone)]
struct AppState { pool: PgPool }

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    assets: Vec<AssetView>,
    invested: Decimal,
    current: Decimal,
    profit: Decimal,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let cfg = config::AppConfig::from_env();
    let pool = PgPool::connect(&cfg.database_url).await.expect("failed to connect database");
    sqlx::migrate!().run(&pool).await.expect("failed to run migrations");

    let state = AppState { pool };
    let app = Router::new()
        .route("/", get(dashboard))
        .route("/health", get(health))
        .route("/assets", post(create_asset))
        .route("/assets/{id}/edit", post(update_asset))
        .route("/assets/{id}/delete", post(delete_asset))
        .route("/api/assets", get(api_assets))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("failed to bind address");
    println!("FinTrack Rust running on http://{addr}");
    axum::serve(listener, app).await.expect("server error");
}

async fn health() -> Json<Value> {
    Json(json!({"status":"ok","service":"fintrack-rust","version":"1.0.0"}))
}

async fn dashboard(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let rows = sqlx::query_as::<_, Asset>("SELECT id, user_id, symbol, name, asset_type, quantity, average_price, current_price FROM assets ORDER BY created_at DESC")
        .fetch_all(&state.pool).await?;
    let assets: Vec<AssetView> = rows.into_iter().map(services::to_view).collect();
    let (invested, current, profit) = services::portfolio_totals(&assets);
    let body = DashboardTemplate { assets, invested, current, profit }.render().map_err(|e| AppError::Template(e.to_string()))?;
    Ok(Html(body))
}

async fn api_assets(State(state): State<AppState>) -> Result<Json<Vec<Asset>>, AppError> {
    let rows = sqlx::query_as::<_, Asset>("SELECT id, user_id, symbol, name, asset_type, quantity, average_price, current_price FROM assets ORDER BY created_at DESC")
        .fetch_all(&state.pool).await?;
    Ok(Json(rows))
}

async fn ensure_demo_user(pool: &PgPool) -> Result<Uuid, AppError> {
    if let Some(id) = sqlx::query_scalar::<_, Uuid>("SELECT id FROM users ORDER BY created_at LIMIT 1").fetch_optional(pool).await? {
        return Ok(id);
    }
    let id = sqlx::query_scalar::<_, Uuid>("INSERT INTO users (name,email,password_hash) VALUES ('Usuário Demo','demo@fintrack.local','demo') RETURNING id")
        .fetch_one(pool).await?;
    Ok(id)
}

async fn create_asset(State(state): State<AppState>, Form(input): Form<AssetForm>) -> Result<Redirect, AppError> {
    validate_asset(&input)?;
    let user_id = ensure_demo_user(&state.pool).await?;
    sqlx::query("INSERT INTO assets (user_id,symbol,name,asset_type,quantity,average_price,current_price) VALUES ($1,$2,$3,$4,$5,$6,$7)")
        .bind(user_id).bind(input.symbol.trim().to_uppercase()).bind(input.name.trim()).bind(input.asset_type.trim())
        .bind(input.quantity).bind(input.average_price).bind(input.current_price)
        .execute(&state.pool).await?;
    Ok(Redirect::to("/"))
}

async fn update_asset(State(state): State<AppState>, Path(id): Path<Uuid>, Form(input): Form<AssetForm>) -> Result<Redirect, AppError> {
    validate_asset(&input)?;
    sqlx::query("UPDATE assets SET symbol=$1,name=$2,asset_type=$3,quantity=$4,average_price=$5,current_price=$6,updated_at=NOW() WHERE id=$7")
        .bind(input.symbol.trim().to_uppercase()).bind(input.name.trim()).bind(input.asset_type.trim())
        .bind(input.quantity).bind(input.average_price).bind(input.current_price).bind(id)
        .execute(&state.pool).await?;
    Ok(Redirect::to("/"))
}

async fn delete_asset(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Redirect, AppError> {
    sqlx::query("DELETE FROM assets WHERE id=$1").bind(id).execute(&state.pool).await?;
    Ok(Redirect::to("/"))
}

fn validate_asset(input: &AssetForm) -> Result<(), AppError> {
    if input.symbol.trim().is_empty() || input.name.trim().is_empty() || input.asset_type.trim().is_empty() {
        return Err(AppError::Validation("preencha símbolo, nome e categoria".into()));
    }
    if input.quantity < Decimal::ZERO || input.average_price < Decimal::ZERO || input.current_price < Decimal::ZERO {
        return Err(AppError::Validation("valores financeiros não podem ser negativos".into()));
    }
    Ok(())
}

#[derive(Debug)]
enum AppError { Database(sqlx::Error), Validation(String), Template(String) }
impl From<sqlx::Error> for AppError { fn from(value: sqlx::Error) -> Self { Self::Database(value) } }
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            AppError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("erro de banco: {e}")),
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, e),
            AppError::Template(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("erro de template: {e}")),
        };
        (status, msg).into_response()
    }
}
