use crate::beverage_type::BeverageType;
use std::collections::HashMap;

pub struct Cashier {
    prices_catalog: HashMap<BeverageType, u32>,
}

impl Cashier {
    pub fn new() -> Cashier {
        let mut prices_catalog = HashMap::new();
        prices_catalog.insert(BeverageType::Coffee, 60);
        prices_catalog.insert(BeverageType::Tea, 40);
        prices_catalog.insert(BeverageType::HotChocolate, 50);
        prices_catalog.insert(BeverageType::OrangeJuice, 60);

        Cashier { prices_catalog }
    }

    pub fn check_payment(
        &self,
        beverage_type: &BeverageType,
        money_amount: u32,
    ) -> BeveragePayment {
        let beverage_price = self.prices_catalog.get(beverage_type).unwrap();

        if money_amount >= *beverage_price {
            BeveragePayment::Ok
        } else {
            BeveragePayment::NotEnoughMoney(beverage_price - money_amount)
        }
    }
}

pub enum BeveragePayment {
    Ok,
    NotEnoughMoney(u32),
}
