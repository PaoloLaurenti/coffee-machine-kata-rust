use super::beverages::beverage::Beverage;

pub(crate) enum UnsuccessfulPayment {
    NotEnoughMoney(u32),
}

pub(crate) struct Cashier {
    cash: Cash,
}

impl Cashier {
    pub(crate) fn new() -> Self {
        Self { cash: Cash::new() }
    }

    pub(crate) fn checkout_payment(
        &mut self,
        beverage: &Beverage,
        money_amount: u32,
    ) -> Result<(), UnsuccessfulPayment> {
        let beverage_price = get_beverage_price(beverage);

        if money_amount >= beverage_price {
            self.cash.deposit(beverage_price);
            Ok(())
        } else {
            Err(UnsuccessfulPayment::NotEnoughMoney(
                beverage_price - money_amount,
            ))
        }
    }

    pub(crate) fn total_money_earned(&self) -> u32 {
        self.cash.total_balance
    }

    pub(crate) fn refund_beverage_payment(&mut self, beverage: &Beverage) {
        let beverage_price = get_beverage_price(beverage);
        self.cash.withdrawn(beverage_price);
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

    fn withdrawn(&mut self, amount: u32) {
        self.total_balance -= amount;
    }
}

fn get_beverage_price(beverage: &Beverage) -> u32 {
    match beverage {
        Beverage::Coffee(_) | Beverage::OrangeJuice => 60,
        Beverage::Tea(_) => 40,
        Beverage::HotChocolate(_) => 50,
    }
}
