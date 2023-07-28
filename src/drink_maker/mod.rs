pub mod drink_maker_beverage_server;
pub mod drink_maker_display;

pub trait DrinkMaker {
    fn execute(&self, command: String);
}
