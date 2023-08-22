use std::collections::HashMap;

use super::{
    beverage::Beverage, beverage_quantity_checker::BeverageQuantityChecker,
    beverage_server::BeverageServer, sugar_amount::SugarAmount,
};

pub(crate) enum BeverageDispsense {
    Ok,
    Shortage,
}

#[derive(Default)]
pub struct DispensedBeveragesHistory {
    pub(crate) quantities: HashMap<Beverage, u32>,
}

impl DispensedBeveragesHistory {
    pub fn record_dispensed_beverage<'a>(&'a mut self, beverage: &'a Beverage) {
        self.quantities
            .entry(beverage.clone())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
}

pub(crate) struct Dispenser<'a> {
    beverage_server: &'a dyn BeverageServer,
    beverage_quantity_checker: &'a dyn BeverageQuantityChecker,
    dispensed_beverages_history: DispensedBeveragesHistory,
}

impl<'a> Dispenser<'a> {
    pub(crate) fn new(
        beverage_server: &'a dyn BeverageServer,
        beverage_quantity_checker: &'a dyn BeverageQuantityChecker,
    ) -> Self {
        Self {
            beverage_server,
            beverage_quantity_checker,
            dispensed_beverages_history: Default::default(),
        }
    }

    pub(crate) fn dispense(
        &mut self,
        beverage: &Beverage,
        sugar_amount: &SugarAmount,
    ) -> BeverageDispsense {
        dbg!("CIAO");
        if self.beverage_quantity_checker.is_empty(beverage) {
            BeverageDispsense::Shortage
        } else {
            dbg!("PIPPA");
            self.beverage_server.serve(beverage, sugar_amount);
            dbg!("PIPPU");
            self.dispensed_beverages_history
                .record_dispensed_beverage(beverage);
            BeverageDispsense::Ok
        }
    }

    pub(crate) fn dispensed_beverages(&self) -> &DispensedBeveragesHistory {
        &self.dispensed_beverages_history
    }
}
