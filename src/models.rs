use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Asset {
    pub id: Uuid,
    pub user_id: Uuid,
    pub symbol: String,
    pub name: String,
    pub asset_type: String,
    pub quantity: Decimal,
    pub average_price: Decimal,
    pub current_price: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct AssetForm {
    pub symbol: String,
    pub name: String,
    pub asset_type: String,
    pub quantity: Decimal,
    pub average_price: Decimal,
    pub current_price: Decimal,
}

#[derive(Debug, Serialize)]
pub struct AssetView {
    pub id: Uuid,
    pub symbol: String,
    pub name: String,
    pub asset_type: String,
    pub quantity: Decimal,
    pub average_price: Decimal,
    pub current_price: Decimal,
    pub invested: Decimal,
    pub current_value: Decimal,
    pub profit_loss: Decimal,
    pub return_percent: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct TransactionForm {
    pub asset_id: Uuid,
    pub transaction_type: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TransactionView {
    pub id: Uuid,
    pub symbol: String,
    pub transaction_type: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct GoalForm {
    pub asset_type: String,
    pub target_percent: Decimal,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AllocationGoal {
    pub asset_type: String,
    pub target_percent: Decimal,
}
