use rust_decimal::Decimal;
use crate::models::{Asset, AssetView};

pub fn to_view(asset: Asset) -> AssetView {
    let invested = asset.quantity * asset.average_price;
    let current_value = asset.quantity * asset.current_price;
    let profit_loss = current_value - invested;
    let return_percent = if invested.is_zero() { Decimal::ZERO } else { (profit_loss / invested) * Decimal::from(100) };

    AssetView {
        symbol: asset.symbol,
        name: asset.name,
        asset_type: asset.asset_type,
        quantity: asset.quantity,
        average_price: asset.average_price,
        current_price: asset.current_price,
        invested,
        current_value,
        profit_loss,
        return_percent,
    }
}

pub fn portfolio_totals(items: &[AssetView]) -> (Decimal, Decimal, Decimal) {
    let invested: Decimal = items.iter().map(|x| x.invested).sum();
    let current: Decimal = items.iter().map(|x| x.current_value).sum();
    (invested, current, current - invested)
}
