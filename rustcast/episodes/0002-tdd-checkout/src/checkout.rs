use crate::item::Item;
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
