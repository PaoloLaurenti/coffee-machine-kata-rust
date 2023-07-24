use std::collections::HashMap;

use crate::{
    drink_maker::DrinkMaker,
    machine::beverage::{Beverage, HotBeverageOption},
    machine::dispenser::{Dispenser, DispensedBeveragesHistory},
    machine::sugar_amount::SugarAmount,
};

pub struct DrinkMakerDispenser<'a> {
    drink_maker: &'a dyn DrinkMaker,
    dispensed_quantities: HashMap<Beverage, u32>
}

impl DrinkMakerDispenser<'_> {
    pub fn new(drink_maker: &impl DrinkMaker) -> DrinkMakerDispenser {
        DrinkMakerDispenser { drink_maker, dispensed_quantities: HashMap::new() }
    }
}

impl Dispenser for DrinkMakerDispenser<'_> {
    fn dispense(&mut self, beverage: Beverage, sugar_amount: &SugarAmount) {
        let drink_maker_cmd = build_beverage_command(&beverage, sugar_amount);
        self.drink_maker.execute(drink_maker_cmd);
        self.dispensed_quantities.entry(beverage).and_modify(|counter| *counter += 1).or_insert(1);
    }

    fn dispensed_beverages(&self) -> DispensedBeveragesHistory {
      DispensedBeveragesHistory::new(self.dispensed_quantities.clone())
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
