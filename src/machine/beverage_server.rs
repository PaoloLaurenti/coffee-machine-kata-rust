use crate::{machine::beverage::Beverage, machine::sugar_amount::SugarAmount};

pub trait BeverageServer {
    fn serve(&self, beverage: &Beverage, sugar_amount: &SugarAmount);
}
