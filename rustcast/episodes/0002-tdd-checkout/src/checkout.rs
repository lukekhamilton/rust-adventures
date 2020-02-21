use crate::item::Item;
use crate::item::Item::*;
use crate::price_list::PriceList;

pub struct Checkout {
    price_list: PriceList,
    items: Vec<Item>,
}
impl Checkout {
    pub fn new(price_list: PriceList) -> Self {
        Self {
            price_list,
            items: Vec::new(),
        }
    }

    pub fn total(&self) -> f64 {
        //        let mut total: f64 = 0.0;
        //        for &item in self.items.iter() {
        //            let price = self.price_list.get(item);
        //            println!("item in checkout: {:?} price: {}", item, price);
        //            total += price
        //        }
        //        total

        self.items
            .iter()
            .map(|&item| self.price_list.get(item))
            .sum()
    }

    pub fn scan(&mut self, item: Item) {
        println!("scanned item: {:?}", item);
        self.items.push(item);
    }
}

mod tests {
    use super::*;

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
    fn test_checkout_with_multiple_items() {
        assert_total(vec![Apple, Tomato, Mango, Tomato, Apple], 3.1)
    }
}
