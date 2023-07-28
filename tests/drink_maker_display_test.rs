use coffee_machine_kata_rust::{
    drink_maker::drink_maker_display::DrinkMakerDisplay, machine::display::Display,
};

use crate::common::drink_maker_spy::DrinkMakerSpy;

mod common;

use test_case::test_case;

#[test_case(1, "M:0.01€" ; "missing one cent message")]
#[test_case(99, "M:0.99€" ; "missing 99 cents message")]
#[test_case(999, "M:9.99€" ; "missing 9,99€ message")]
#[test_case(9999, "M:99.99€" ; "missing 99,99€ message")]
fn show_missing_money_message(missing_money: u32, expected_message: &str) {
    let drink_maker_spy = DrinkMakerSpy::new();
    let drink_maker_display = DrinkMakerDisplay::new(&drink_maker_spy);

    drink_maker_display.show_missing_money_message(missing_money);

    let drink_maker_cmds = drink_maker_spy.get_received_commands();
    assert_eq!(drink_maker_cmds, vec![(String::from(expected_message))])
}
