use crate::{drink_maker::DrinkMaker, display::Display};

pub struct DrinkMakerDisplay<'a> {
    drink_maker: &'a dyn DrinkMaker,
}

impl DrinkMakerDisplay<'_> {
  pub fn new(drink_maker: &dyn DrinkMaker) -> DrinkMakerDisplay {
    DrinkMakerDisplay { drink_maker }
  }
}

impl Display for DrinkMakerDisplay<'_> {
    fn show_missing_money_message(&self, missing_money: u32) {
        let formatted_missing_money = missing_money as f32 / 100.0;
        self.drink_maker
            .execute(format!("M:{formatted_missing_money}€"));
    }
}
