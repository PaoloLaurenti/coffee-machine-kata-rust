use coffee_machine_kata_rust::{
    drink_maker::drink_maker_display::DrinkMakerDisplay,
    machine::{
        beverage::{Beverage, HotBeverageOption},
        display::Display,
    },
};

mod common;

use test_case::test_case;

use crate::common::drink_maker_test_double::DrinkMakerTestDouble;

#[test_case(1, "M:0.01€" ; "missing one cent message")]
#[test_case(99, "M:0.99€" ; "missing 99 cents message")]
#[test_case(999, "M:9.99€" ; "missing 9,99€ message")]
#[test_case(9999, "M:99.99€" ; "missing 99,99€ message")]
fn show_missing_money_message(missing_money: u32, expected_message: &str) {
    let drink_maker_test_double = DrinkMakerTestDouble::new();
    let drink_maker_display = DrinkMakerDisplay::new(&drink_maker_test_double);

    drink_maker_display.show_missing_money_message(missing_money);

    let drink_maker_cmds = drink_maker_test_double.spied_received_commands();
    assert_eq!(drink_maker_cmds, vec![(String::from(expected_message))])
}

#[test_case(Beverage::Coffee(HotBeverageOption::Standard), "M:Sorry, coffee is empty." ; "coffee (standard) empty message")]
#[test_case(Beverage::Coffee(HotBeverageOption::ExtraHot), "M:Sorry, coffee is empty." ; "coffee (extra hot) empty message")]
#[test_case(Beverage::Tea(HotBeverageOption::Standard), "M:Sorry, tea is empty." ; "tea (standard) empty message")]
#[test_case(Beverage::Tea(HotBeverageOption::ExtraHot), "M:Sorry, tea is empty." ; "tea (extra hot) empty message")]
#[test_case(Beverage::HotChocolate(HotBeverageOption::Standard), "M:Sorry, hot chocolate is empty." ; "hot chocolate (standard) empty message")]
#[test_case(Beverage::HotChocolate(HotBeverageOption::ExtraHot), "M:Sorry, hot chocolate is empty." ; "hot chocolate (extra hot) empty message")]
#[test_case(Beverage::OrangeJuice, "M:Sorry, orange juice is empty." ; "orane juice empty message")]
fn show_beverage_shortage_message(beverage: Beverage, expected_missing_beverage_message: &str) {
    let drink_maker_test_double = DrinkMakerTestDouble::new();
    let drink_maker_display = DrinkMakerDisplay::new(&drink_maker_test_double);

    drink_maker_display.show_beverage_shortage_message(&beverage);

    let drink_maker_cmds = drink_maker_test_double.spied_received_commands();
    assert_eq!(
        drink_maker_cmds,
        vec![(String::from(expected_missing_beverage_message))]
    )
}
