use crate::{beverage_type::BeverageType, drink_maker::DrinkMaker, sugar_amount::SugarAmount};

pub struct DrinkMakerProxy<'a> {
    drink_maker: &'a dyn DrinkMaker,
}

impl DrinkMakerProxy<'_> {
    pub fn new(drink_maker: &dyn DrinkMaker) -> DrinkMakerProxy {
        DrinkMakerProxy { drink_maker }
    }

    pub fn dispense(&self, beverage_type: &BeverageType, sugar_amount: &SugarAmount) {
        let drink_maker_cmd =
            DrinkMakerProxy::build_drink_maker_beverage_command(beverage_type, sugar_amount);
        self.drink_maker.execute(drink_maker_cmd)
    }

    pub fn show_missing_money_message(&self, missing_money: u32) {
        let formatted_missing_money = missing_money as f32 / 100.0;
        self.drink_maker
            .execute(format!("M:{formatted_missing_money}€"));
    }

    fn build_drink_maker_beverage_command(
        beverage_type: &BeverageType,
        sugar_amount: &SugarAmount,
    ) -> String {
        let beverage_cmd_part = match beverage_type {
            BeverageType::Coffee => "C",
            BeverageType::Tea => "T",
            BeverageType::HotChocolate => "H",
        };

        let (sugar_amount_cmd_part, stick_cmd_part) = match sugar_amount {
            SugarAmount::Zero => ("", ""),
            SugarAmount::One => ("1", "0"),
            SugarAmount::Two => ("2", "0"),
        };

        format!("{beverage_cmd_part}:{sugar_amount_cmd_part}:{stick_cmd_part}")
    }
}
