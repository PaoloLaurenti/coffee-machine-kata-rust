use crate::machine::beverage::Beverage;

pub enum BeveragePayment {
    Ok,
    NotEnoughMoney(u32),
}

pub struct Cashier {
    cash: Cash,
}

impl Cashier {
    pub fn new() -> Self {
        Self { cash: Cash::new() }
    }

    pub fn checkout_payment(&mut self, beverage: &Beverage, money_amount: u32) -> BeveragePayment {
        let beverage_price = get_beverage_price(beverage);

        if money_amount >= beverage_price {
            self.cash.deposit(beverage_price);
            BeveragePayment::Ok
        } else {
            BeveragePayment::NotEnoughMoney(beverage_price - money_amount)
        }
    }

    pub fn total_money_earned(&self) -> u32 {
        self.cash.total_balance
    }
}

struct Cash {
    total_balance: u32,
}

impl Cash {
    fn new() -> Self {
        Self { total_balance: 0 }
    }

    fn deposit(&mut self, amount: u32) {
        self.total_balance += amount;
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
