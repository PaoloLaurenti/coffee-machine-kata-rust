use crate::{beverage::{Beverage, HotBeverageOption}, drink_maker::DrinkMaker, sugar_amount::SugarAmount};

pub struct DrinkMakerProxy<'a> {
    drink_maker: &'a dyn DrinkMaker,
}

impl DrinkMakerProxy<'_> {
    pub fn new(drink_maker: &dyn DrinkMaker) -> DrinkMakerProxy {
        DrinkMakerProxy { drink_maker }
    }

    pub fn dispense(&self, beverage: &Beverage, sugar_amount: &SugarAmount) {
        let drink_maker_cmd = DrinkMakerProxy::build_beverage_command(beverage, sugar_amount);
        self.drink_maker.execute(drink_maker_cmd)
    }

    pub fn show_missing_money_message(&self, missing_money: u32) {
        let formatted_missing_money = missing_money as f32 / 100.0;
        self.drink_maker
            .execute(format!("M:{formatted_missing_money}€"));
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
}
