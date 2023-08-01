mod common;

use coffee_machine_kata_rust::{
    drink_maker::drink_maker_beverage_server::DrinkMakerBeverageServer,
    machine::{
        beverage::{Beverage, HotBeverageOption},
        beverage_server::BeverageServer,
        sugar_amount::SugarAmount,
    },
};
use test_case::test_case;

use crate::common::drink_maker_test_double::DrinkMakerTestDouble;

#[test_case(Beverage::Coffee(HotBeverageOption::Standard), "C::" ; "coffee")]
#[test_case(Beverage::Coffee(HotBeverageOption::ExtraHot), "Ch::" ; "extra hot coffee")]
#[test_case(Beverage::Tea(HotBeverageOption::Standard), "T::" ; "tea")]
#[test_case(Beverage::Tea(HotBeverageOption::ExtraHot), "Th::" ; "extra hot tea")]
#[test_case(Beverage::HotChocolate(HotBeverageOption::Standard), "H::" ; "hot chocolate")]
#[test_case(Beverage::HotChocolate(HotBeverageOption::ExtraHot), "Hh::" ; "extra hot hot chocolate")]
#[test_case(Beverage::OrangeJuice, "O::" ; "Orange juice")]
fn serve_beverages_with_no_sugar(beverage: Beverage, expected_drink_maker_cmd: &str) {
    let drink_maker_test_double = DrinkMakerTestDouble::new();
    let drink_maker_beverage_server = DrinkMakerBeverageServer::new(&drink_maker_test_double);

    drink_maker_beverage_server.serve(&beverage, &SugarAmount::Zero);

    let drink_maker_cmds = drink_maker_test_double.spied_received_commands();
    assert_eq!(
        drink_maker_cmds,
        vec![(String::from(expected_drink_maker_cmd))]
    )
}

#[test_case(Beverage::OrangeJuice, SugarAmount::One, "O:1:0" ; "Orange juice with one sugar")]
#[test_case(Beverage::Coffee(HotBeverageOption::Standard), SugarAmount::Two, "C:2:0" ; "Coffee with two sugars")]
fn serve_beverages_with_sugar_and_stick(
    beverage: Beverage,
    sugar_amount: SugarAmount,
    expected_drink_maker_cmd: &str,
) {
    let drink_maker_test_double = DrinkMakerTestDouble::new();
    let drink_maker_beverage_server = DrinkMakerBeverageServer::new(&drink_maker_test_double);

    drink_maker_beverage_server.serve(&beverage, &sugar_amount);

    let drink_maker_cmds = drink_maker_test_double.spied_received_commands();
    assert_eq!(
        drink_maker_cmds,
        vec![(String::from(expected_drink_maker_cmd))]
    )
}
