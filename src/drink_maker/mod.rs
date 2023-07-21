pub mod drink_maker_dispenser;
pub mod drink_maker_display;

pub trait DrinkMaker {
    fn execute(&self, command: String);
}
