use rust_decimal::Decimal;

pub fn buy_position(
    current_quantity: Decimal,
    current_average: Decimal,
    buy_quantity: Decimal,
    buy_price: Decimal,
) -> Result<(Decimal, Decimal), String> {
    if buy_quantity <= Decimal::ZERO || buy_price < Decimal::ZERO {
        return Err("quantidade deve ser positiva e preço não pode ser negativo".into());
    }
    let total_before = current_quantity * current_average;
    let total_buy = buy_quantity * buy_price;
    let new_quantity = current_quantity + buy_quantity;
    let new_average = if new_quantity.is_zero() {
        Decimal::ZERO
    } else {
        (total_before + total_buy) / new_quantity
    };
    Ok((new_quantity, new_average))
}

pub fn sell_position(
    current_quantity: Decimal,
    sell_quantity: Decimal,
) -> Result<Decimal, String> {
    if sell_quantity <= Decimal::ZERO {
        return Err("quantidade de venda deve ser positiva".into());
    }
    if sell_quantity > current_quantity {
        return Err("venda maior que a quantidade disponível".into());
    }
    Ok(current_quantity - sell_quantity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn recalculates_average_price_on_buy() {
        let (qty, avg) = buy_position(
            Decimal::from(10),
            Decimal::from(20),
            Decimal::from(10),
            Decimal::from(30),
        ).unwrap();
        assert_eq!(qty, Decimal::from(20));
        assert_eq!(avg, Decimal::from(25));
    }

    #[test]
    fn blocks_sell_above_available_quantity() {
        let err = sell_position(Decimal::from(5), Decimal::from(6)).unwrap_err();
        assert!(err.contains("quantidade disponível"));
    }
}
