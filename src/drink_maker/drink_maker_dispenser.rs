use crate::{
    drink_maker::DrinkMaker,
    machine::beverage::{Beverage, HotBeverageOption},
    machine::sugar_amount::SugarAmount,
    machine::{
        beverage_quantity_checker::BeverageQuantityChecker,
        dispenser::{BeverageDispsense, DispensedBeveragesHistory, Dispenser},
    },
};

pub struct DrinkMakerDispenser<'a> {
    drink_maker: &'a dyn DrinkMaker,
    beverage_quantity_checker: &'a dyn BeverageQuantityChecker,
    dispensed_beverages_history: DispensedBeveragesHistory,
}

impl DrinkMakerDispenser<'_> {
    pub fn new<'a>(
        drink_maker: &'a impl DrinkMaker,
        beverage_quantity_checker: &'a impl BeverageQuantityChecker,
    ) -> DrinkMakerDispenser<'a> {
        DrinkMakerDispenser {
            drink_maker,
            beverage_quantity_checker,
            dispensed_beverages_history: DispensedBeveragesHistory::new(),
        }
    }

    fn send_beverage_request(&mut self, beverage: &Beverage, sugar_amount: &SugarAmount) {
        let drink_maker_cmd = build_beverage_command(beverage, sugar_amount);
        self.drink_maker.execute(drink_maker_cmd);
    }

    fn record_dispensed_beverage(&mut self, beverage: &Beverage) {
        self.dispensed_beverages_history
            .record_dispensed_beverage(beverage)
    }
}

impl Dispenser for DrinkMakerDispenser<'_> {
    fn dispense(&mut self, beverage: &Beverage, sugar_amount: &SugarAmount) -> BeverageDispsense {
        if self.beverage_quantity_checker.is_empty(beverage) {
            BeverageDispsense::Shortage
        } else {
            self.send_beverage_request(beverage, sugar_amount);
            self.record_dispensed_beverage(beverage);
            BeverageDispsense::Ok
        }
    }

    fn dispensed_beverages(&self) -> &DispensedBeveragesHistory {
        &self.dispensed_beverages_history
    }
}

fn build_beverage_command(beverage: &Beverage, sugar_amount: &SugarAmount) -> String {
    let beverage_cmd_part = match beverage {
        Beverage::Coffee(HotBeverageOption::Standard) => "C",
        Beverage::Coffee(HotBeverageOption::ExtraHot) => "Ch",
        Beverage::Tea(HotBeverageOption::Standard) => "T",
        Beverage::Tea(HotBeverageOption::ExtraHot) => "Th",
        Beverage::HotChocolate(HotBeverageOption::Standard) => "H",
        Beverage::HotChocolate(HotBeverageOption::ExtraHot) => "Hh",
        Beverage::OrangeJuice => "O",
    };

    let (sugar_amount_cmd_part, stick_cmd_part) = match sugar_amount {
        SugarAmount::Zero => ("", ""),
        SugarAmount::One => ("1", "0"),
        SugarAmount::Two => ("2", "0"),
    };

    format!("{beverage_cmd_part}:{sugar_amount_cmd_part}:{stick_cmd_part}")
}
