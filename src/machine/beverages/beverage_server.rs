use super::{beverage::Beverage, sugar_amount::SugarAmount};

pub trait BeverageServer {
    fn serve(&self, beverage: &Beverage, sugar_amount: &SugarAmount);
}
