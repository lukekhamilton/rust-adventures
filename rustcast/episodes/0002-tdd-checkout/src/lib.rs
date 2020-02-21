mod checkout;
mod item;
mod price_list;

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    use super::*;

    use checkout::Checkout;
    use item::Item;
    use item::Item::*;
    use price_list::PriceList;

    fn assert_total(items: Vec<Item>, expected_total: f64) {
        let mut checkout = build_check_out();
        for &item in items.iter() {
            checkout.scan(item);
        }
        let actual_total = checkout.total();
        assert_eq!(actual_total, expected_total);
    }

    fn build_check_out() -> Checkout {
        let mut price_list = PriceList::new();
        price_list.set(Apple, 0.40);
        price_list.set(Tomato, 0.25);
        price_list.set(Mango, 1.80);

        Checkout::new(price_list)
    }

    #[test]
    fn test_checkout_empty() {
        assert_total(vec![], 0.0);
    }

    #[test]
    fn test_checkout_with_1_apple() {
        assert_total(vec![Apple], 0.4);
    }

    #[test]
    fn test_checkout_with_1_tomato() {
        assert_total(vec![Tomato], 0.25);
    }

    #[test]
    fn test_checkout_with_1_mango() {
        assert_total(vec![Mango], 1.8);
    }

    #[test]
    fn test_price_list_default() {
        let price_list = PriceList::new();
        assert_eq!(price_list.get(Apple), 0.0);
        assert_eq!(price_list.get(Tomato), 0.0);
        assert_eq!(price_list.get(Mango), 0.0);
    }

    #[test]
    fn test_price_list_set_and_get_prices() {
        let mut price_list = PriceList::new();
        price_list.set(Apple, 0.40);
        price_list.set(Tomato, 0.25);
        price_list.set(Mango, 1.80);

        assert_eq!(price_list.get(Apple), 0.40);
        assert_eq!(price_list.get(Tomato), 0.25);
        assert_eq!(price_list.get(Mango), 1.80);
    }

    #[test]
    fn test_checkout_with_multiple_items() {
        assert_total(vec![Apple, Tomato, Mango, Tomato, Apple], 3.1)
    }
}
