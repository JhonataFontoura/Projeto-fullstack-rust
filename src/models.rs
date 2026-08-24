use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
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
