use super::{beverage::Beverage, sugar_amount::SugarAmount};

pub struct BeverageRequest<'a> {
    pub beverage: &'a Beverage,
    pub sugar_amount: &'a SugarAmount,
    pub money_amount: u32,
}

impl BeverageRequest<'_> {
    pub fn new<'a>(
        beverage: &'a Beverage,
        sugar_amount: &'a SugarAmount,
        money_amount: u32,
    ) -> BeverageRequest<'a> {
        BeverageRequest {
            beverage,
            sugar_amount,
            money_amount,
        }
    }
}
