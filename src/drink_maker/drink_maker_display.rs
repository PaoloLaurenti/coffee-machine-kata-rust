use crate::{
    drink_maker::DrinkMaker,
    machine::{beverage::Beverage, display::Display},
};

pub struct DrinkMakerDisplay<'a> {
    drink_maker: &'a dyn DrinkMaker,
}

impl DrinkMakerDisplay<'_> {
    pub fn new(drink_maker: &impl DrinkMaker) -> DrinkMakerDisplay {
        DrinkMakerDisplay { drink_maker }
    }
}

impl Display for DrinkMakerDisplay<'_> {
    fn show_missing_money_message(&self, missing_money: u32) {
        let formatted_missing_money = missing_money as f32 / 100.0;
        self.drink_maker
            .execute(format!("M:{formatted_missing_money}€"));
    }

    fn show_beverage_shortage_message(&self, beverage: &Beverage) {
        self.drink_maker
            .execute(format!("M:Sorry, {beverage} is empty."));
    }
}

impl std::fmt::Display for Beverage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Beverage::Coffee(_) => write!(f, "coffee"),
            Beverage::Tea(_) => write!(f, "tea"),
            Beverage::HotChocolate(_) => write!(f, "hot chocolate"),
            Beverage::OrangeJuice => write!(f, "orange juice"),
        }
    }
}
