use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Form, Json, Router,
};
use axum_extra::extract::cookie::CookieJar;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod auth;
mod config;
mod models;
mod services;

use models::{Asset, AssetForm, AssetView};

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    jwt_secret: String,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    assets: Vec<AssetView>,
    invested: Decimal,
    current: Decimal,
    profit: Decimal,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate;

#[derive(Debug, Deserialize)]
struct RegisterForm {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let cfg = config::AppConfig::from_env();
    let pool = PgPool::connect(&cfg.database_url)
        .await
        .expect("failed to connect database");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to run migrations");

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "change-me-in-production".to_string());

    let state = AppState { pool, jwt_secret };
    let app = Router::new()
        .route("/", get(dashboard))
        .route("/health", get(health))
        .route("/login", get(login_page).post(login))
        .route("/register", get(register_page).post(register))
        .route("/logout", post(logout))
        .route("/assets", post(create_asset))
        .route("/assets/{id}/edit", post(update_asset))
        .route("/assets/{id}/delete", post(delete_asset))
        .route("/api/assets", get(api_assets))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind address");
    println!("FinTrack Rust running on http://{addr}");
    axum::serve(listener, app).await.expect("server error");
}

async fn health() -> Json<Value> {
    Json(json!({"status":"ok","service":"fintrack-rust","version":"1.0.1"}))
}

async fn login_page() -> Result<Html<String>, AppError> {
    Ok(Html(LoginTemplate.render().map_err(|e| AppError::Template(e.to_string()))?))
}

async fn register_page() -> Result<Html<String>, AppError> {
    Ok(Html(RegisterTemplate.render().map_err(|e| AppError::Template(e.to_string()))?))
}

async fn register(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(input): Form<RegisterForm>,
) -> Result<(CookieJar, Redirect), AppError> {
    if input.name.trim().len() < 2 {
        return Err(AppError::Validation("nome deve ter pelo menos 2 caracteres".into()));
    }
    if !input.email.contains('@') {
        return Err(AppError::Validation("e-mail inválido".into()));
    }
    if input.password.len() < 8 {
        return Err(AppError::Validation("senha deve ter pelo menos 8 caracteres".into()));
    }

    let password_hash = auth::hash_password(&input.password).map_err(AppError::Auth)?;
    let user_id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO users (name,email,password_hash) VALUES ($1,$2,$3) RETURNING id",
    )
    .bind(input.name.trim())
    .bind(input.email.trim().to_lowercase())
    .bind(password_hash)
    .fetch_one(&state.pool)
    .await?;

    let cookie = auth::session_cookie(user_id, &state.jwt_secret).map_err(AppError::Auth)?;
    Ok((jar.add(cookie), Redirect::to("/")))
}

async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(input): Form<LoginForm>,
) -> Result<(CookieJar, Redirect), AppError> {
    let row = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT id, password_hash FROM users WHERE email=$1",
    )
    .bind(input.email.trim().to_lowercase())
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("credenciais inválidas".into()))?;

    if !auth::verify_password(&input.password, &row.1) {
        return Err(AppError::Unauthorized("credenciais inválidas".into()));
    }

    let cookie = auth::session_cookie(row.0, &state.jwt_secret).map_err(AppError::Auth)?;
    Ok((jar.add(cookie), Redirect::to("/")))
}

async fn logout(jar: CookieJar) -> (CookieJar, Redirect) {
    (jar.remove(auth::clear_session_cookie()), Redirect::to("/login"))
}

async fn dashboard(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Response, AppError> {
    let Some(user_id) = auth::current_user_id(&jar, &state.jwt_secret) else {
        return Ok(Redirect::to("/login").into_response());
    };

    let rows = sqlx::query_as::<_, Asset>(
        "SELECT id, user_id, symbol, name, asset_type, quantity, average_price, current_price FROM assets WHERE user_id=$1 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(&state.pool)
    .await?;

    let assets: Vec<AssetView> = rows.into_iter().map(services::to_view).collect();
    let (invested, current, profit) = services::portfolio_totals(&assets);
    let body = DashboardTemplate { assets, invested, current, profit }
        .render()
        .map_err(|e| AppError::Template(e.to_string()))?;
    Ok(Html(body).into_response())
}

async fn api_assets(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<Vec<Asset>>, AppError> {
    let user_id = require_user(&jar, &state)?;
    let rows = sqlx::query_as::<_, Asset>(
        "SELECT id, user_id, symbol, name, asset_type, quantity, average_price, current_price FROM assets WHERE user_id=$1 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(rows))
}

async fn create_asset(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(input): Form<AssetForm>,
) -> Result<Redirect, AppError> {
    validate_asset(&input)?;
    let user_id = require_user(&jar, &state)?;
    sqlx::query("INSERT INTO assets (user_id,symbol,name,asset_type,quantity,average_price,current_price) VALUES ($1,$2,$3,$4,$5,$6,$7)")
        .bind(user_id)
        .bind(input.symbol.trim().to_uppercase())
        .bind(input.name.trim())
        .bind(input.asset_type.trim())
        .bind(input.quantity)
        .bind(input.average_price)
        .bind(input.current_price)
        .execute(&state.pool)
        .await?;
    Ok(Redirect::to("/"))
}

async fn update_asset(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
    Form(input): Form<AssetForm>,
) -> Result<Redirect, AppError> {
    validate_asset(&input)?;
    let user_id = require_user(&jar, &state)?;
    sqlx::query("UPDATE assets SET symbol=$1,name=$2,asset_type=$3,quantity=$4,average_price=$5,current_price=$6,updated_at=NOW() WHERE id=$7 AND user_id=$8")
        .bind(input.symbol.trim().to_uppercase())
        .bind(input.name.trim())
        .bind(input.asset_type.trim())
        .bind(input.quantity)
        .bind(input.average_price)
        .bind(input.current_price)
        .bind(id)
        .bind(user_id)
        .execute(&state.pool)
        .await?;
    Ok(Redirect::to("/"))
}

async fn delete_asset(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
) -> Result<Redirect, AppError> {
    let user_id = require_user(&jar, &state)?;
    sqlx::query("DELETE FROM assets WHERE id=$1 AND user_id=$2")
        .bind(id)
        .bind(user_id)
        .execute(&state.pool)
        .await?;
    Ok(Redirect::to("/"))
}

fn require_user(jar: &CookieJar, state: &AppState) -> Result<Uuid, AppError> {
    auth::current_user_id(jar, &state.jwt_secret)
        .ok_or_else(|| AppError::Unauthorized("faça login para continuar".into()))
}

fn validate_asset(input: &AssetForm) -> Result<(), AppError> {
    if input.symbol.trim().is_empty() || input.name.trim().is_empty() || input.asset_type.trim().is_empty() {
        return Err(AppError::Validation("preencha símbolo, nome e categoria".into()));
    }
    if input.quantity < Decimal::ZERO
        || input.average_price < Decimal::ZERO
        || input.current_price < Decimal::ZERO
    {
        return Err(AppError::Validation("valores financeiros não podem ser negativos".into()));
    }
    Ok(())
}

#[derive(Debug)]
enum AppError {
    Database(sqlx::Error),
    Validation(String),
    Unauthorized(String),
    Auth(String),
    Template(String),
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::Database(e) => {
                if let Some(db) = e.as_database_error() {
                    if db.code().as_deref() == Some("23505") {
                        return (StatusCode::CONFLICT, "e-mail já cadastrado").into_response();
                    }
                }
                (StatusCode::INTERNAL_SERVER_ERROR, "erro interno de banco de dados".to_string())
            }
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, e),
            AppError::Unauthorized(e) => (StatusCode::UNAUTHORIZED, e),
            AppError::Auth(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("erro de autenticação: {e}")),
            AppError::Template(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("erro de template: {e}")),
        };
        (status, msg).into_response()
    }
}
