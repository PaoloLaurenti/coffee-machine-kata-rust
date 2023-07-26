use std::collections::HashMap;

use crate::{machine::beverage::Beverage, machine::sugar_amount::SugarAmount};

pub trait Dispenser {
    fn dispense(&mut self, beverage: &Beverage, sugar_amount: &SugarAmount) -> BeverageDispsense;
    fn dispensed_beverages(&self) -> &DispensedBeveragesHistory;
}

pub enum BeverageDispsense {
    Ok,
    Shortage,
}

pub struct DispensedBeveragesHistory {
    pub quantities: HashMap<Beverage, u32>,
}

impl DispensedBeveragesHistory {
    pub fn new() -> Self {
        Self {
            quantities: HashMap::new(),
        }
    }

    pub fn record_dispensed_beverage<'a>(&'a mut self, beverage: &'a Beverage) {
        self.quantities
            .entry(beverage.clone())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
}

impl Default for DispensedBeveragesHistory {
    fn default() -> Self {
        Self::new()
    }
}
