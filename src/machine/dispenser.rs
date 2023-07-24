use std::collections::HashMap;

use crate::{machine::beverage::Beverage, machine::sugar_amount::SugarAmount};

pub trait Dispenser {
    fn dispense(&mut self, beverage: Beverage, sugar_amount: &SugarAmount);
    fn dispensed_beverages(&self) -> DispensedBeveragesHistory;
}

pub struct DispensedBeveragesHistory {
    pub quantities: HashMap<Beverage, u32>,
}

impl DispensedBeveragesHistory {
    pub fn new(quantities: HashMap<Beverage, u32>) -> Self {
        Self { quantities }
    }
}
