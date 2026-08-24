use crate::models::AssetView;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct AllocationMetric {
    pub asset_type: String,
    pub current_value: Decimal,
    pub percent: Decimal,
}

#[derive(Debug, Clone, Serialize)]
pub struct GoalMetric {
    pub asset_type: String,
    pub current_percent: Decimal,
    pub target_percent: Decimal,
    pub difference: Decimal,
}

pub fn allocation_by_type(items: &[AssetView]) -> Vec<AllocationMetric> {
    let total: Decimal = items.iter().map(|item| item.current_value).sum();
    let mut grouped: BTreeMap<String, Decimal> = BTreeMap::new();

    for item in items {
        *grouped.entry(item.asset_type.clone()).or_insert(Decimal::ZERO) += item.current_value;
    }

    grouped
        .into_iter()
        .map(|(asset_type, current_value)| {
            let percent = if total.is_zero() {
                Decimal::ZERO
            } else {
                (current_value / total) * Decimal::from(100)
            };
            AllocationMetric { asset_type, current_value, percent }
        })
        .collect()
}

pub fn compare_goals(
    allocation: &[AllocationMetric],
    goals: &[(String, Decimal)],
) -> Vec<GoalMetric> {
    goals
        .iter()
        .map(|(asset_type, target)| {
            let current = allocation
                .iter()
                .find(|item| item.asset_type == *asset_type)
                .map(|item| item.percent)
                .unwrap_or(Decimal::ZERO);
            GoalMetric {
                asset_type: asset_type.clone(),
                current_percent: current,
                target_percent: *target,
                difference: current - *target,
            }
        })
        .collect()
}

pub fn percent_width(percent: Decimal) -> u32 {
    percent.round().to_u32().unwrap_or(0).min(100)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn asset(kind: &str, value: i64) -> AssetView {
        AssetView {
            id: Uuid::new_v4(),
            symbol: kind.into(),
            name: kind.into(),
            asset_type: kind.into(),
            quantity: Decimal::ONE,
            average_price: Decimal::from(value),
            current_price: Decimal::from(value),
            invested: Decimal::from(value),
            current_value: Decimal::from(value),
            profit_loss: Decimal::ZERO,
            return_percent: Decimal::ZERO,
        }
    }

    #[test]
    fn calculates_allocation_percentages() {
        let result = allocation_by_type(&[asset("Ações", 75), asset("FIIs", 25)]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].percent + result[1].percent, Decimal::from(100));
    }

    #[test]
    fn compares_current_allocation_with_goal() {
        let allocation = allocation_by_type(&[asset("Ações", 60), asset("FIIs", 40)]);
        let goals = vec![("Ações".into(), Decimal::from(50))];
        let result = compare_goals(&allocation, &goals);
        assert_eq!(result[0].difference, Decimal::from(10));
    }
}
