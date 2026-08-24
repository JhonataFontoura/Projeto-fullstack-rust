use fintrack_rust::portfolio::{buy_position, sell_position};
use rust_decimal::Decimal;

#[test]
fn buy_recalculates_weighted_average() {
    let (qty, avg) = buy_position(
        Decimal::from(10),
        Decimal::from(20),
        Decimal::from(5),
        Decimal::from(40),
    ).unwrap();

    assert_eq!(qty, Decimal::from(15));
    assert_eq!(avg, Decimal::from(80) / Decimal::from(3));
}

#[test]
fn sell_keeps_position_non_negative() {
    let qty = sell_position(Decimal::from(10), Decimal::from(4)).unwrap();
    assert_eq!(qty, Decimal::from(6));
}

#[test]
fn sell_above_available_is_rejected() {
    assert!(sell_position(Decimal::from(2), Decimal::from(3)).is_err());
}

#[test]
fn zero_or_negative_buy_is_rejected() {
    assert!(buy_position(Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, Decimal::from(10)).is_err());
}
