use crate::{machine::beverage::Beverage, machine::sugar_amount::SugarAmount};

pub trait Dispenser {
    fn dispense(&self, beverage: &Beverage, sugar_amount: &SugarAmount);
}
