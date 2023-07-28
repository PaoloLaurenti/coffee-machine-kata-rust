use crate::{
    drink_maker::DrinkMaker,
    machine::beverage::{Beverage, HotBeverageOption},
    machine::beverage_server::BeverageServer,
    machine::sugar_amount::SugarAmount,
};

pub struct DrinkMakerBeverageServer<'a> {
    drink_maker: &'a dyn DrinkMaker,
}

impl DrinkMakerBeverageServer<'_> {
    pub fn new(drink_maker: &impl DrinkMaker) -> DrinkMakerBeverageServer<'_> {
        DrinkMakerBeverageServer { drink_maker }
    }
}

impl BeverageServer for DrinkMakerBeverageServer<'_> {
    fn serve(&self, beverage: &Beverage, sugar_amount: &SugarAmount) {
        let drink_maker_cmd = build_beverage_command(beverage, sugar_amount);
        self.drink_maker.execute(drink_maker_cmd);
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
