use crate::machine::beverage::Beverage;

pub enum BeveragePayment {
    Ok,
    NotEnoughMoney(u32),
}

pub fn check_payment(beverage: &Beverage, money_amount: u32) -> BeveragePayment {
    let beverage_price = get_beverage_price(beverage);

    if money_amount >= beverage_price {
        BeveragePayment::Ok
    } else {
        BeveragePayment::NotEnoughMoney(beverage_price - money_amount)
    }
}

fn get_beverage_price(beverage: &Beverage) -> u32 {
    match beverage {
        Beverage::Coffee(_) => 60,
        Beverage::Tea(_) => 40,
        Beverage::HotChocolate(_) => 50,
        Beverage::OrangeJuice => 60,
    }
}
