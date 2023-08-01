use super::beverage::Beverage;

pub trait BeverageQuantityChecker {
    fn is_empty(&self, beverage: &Beverage) -> bool;
}
