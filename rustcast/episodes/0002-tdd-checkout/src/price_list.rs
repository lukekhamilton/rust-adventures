use crate::item::Item;
use crate::item::Item::*;

pub struct PriceList {
    apple: f64,
    tomato: f64,
    mango: f64,
}

impl PriceList {
    pub fn new() -> Self {
        Self {
            apple: 0.0,
            tomato: 0.0,
            mango: 0.0,
        }
    }

    pub fn set(&mut self, item: Item, price: f64) {
        match item {
            Apple => self.apple = price,
            Tomato => self.tomato = price,
            Mango => self.mango = price,
        }
    }

    pub fn get(&self, item: Item) -> f64 {
        match item {
            Apple => self.apple,
            Tomato => self.tomato,
            Mango => self.mango,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
}
