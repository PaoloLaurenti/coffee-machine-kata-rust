use super::beverage::Beverage;

pub trait Display {
    fn show_missing_money_message(&self, missing_money: u32);
    fn show_beverage_shortage_message(&self, beverage: &Beverage);
}
