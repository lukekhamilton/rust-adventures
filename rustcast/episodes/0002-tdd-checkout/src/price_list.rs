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
        println!("set item: {:?}, price: {}", item, price);
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
