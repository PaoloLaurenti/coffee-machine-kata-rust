use coffee_machine_kata_rust::machine::{
    beverage::Beverage, beverage_quantity_checker::BeverageQuantityChecker,
};

pub(crate) struct BeverageQuantityCheckerFake {
    always_empty: bool,
}

impl BeverageQuantityCheckerFake {
    pub(crate) fn new(always_empty: bool) -> Self {
        Self { always_empty }
    }
}

impl BeverageQuantityChecker for BeverageQuantityCheckerFake {
    fn is_empty(&self, _beverage: &Beverage) -> bool {
        self.always_empty
    }
}
