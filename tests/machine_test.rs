mod common;

use crate::common::{
    drink_maker_spy::DrinkMakerSpy, dummy_notifier::DummyNotifier,
    dummy_reports_printer::DummyReportsPrinter,
};
use coffee_machine_kata_rust::{
    drink_maker::{
        drink_maker_beverage_server::DrinkMakerBeverageServer,
        drink_maker_display::DrinkMakerDisplay,
    },
    machine::{
        beverage::Beverage, beverage::HotBeverageOption,
        beverage_quantity_checker::BeverageQuantityChecker, beverage_request::BeverageRequest,
        sugar_amount::SugarAmount, Machine,
    },
};
use test_case::test_case;

const ENOUGH_MONEY: u32 = 100;

struct BeverageQuantityCheckerFake {
    always_empty: bool,
}

impl BeverageQuantityCheckerFake {
    pub(crate) fn new(always_empty: bool) -> Self {
        Self { always_empty }
    }
}

impl BeverageQuantityChecker for BeverageQuantityCheckerFake {
    fn is_empty(&self, _beverage: &Beverage) -> bool {
        self.always_empty
    }
}

#[test_case(Beverage::Coffee(HotBeverageOption::Standard), SugarAmount::One, "C:1:0" ; "coffee")]
#[test_case(Beverage::Coffee(HotBeverageOption::ExtraHot), SugarAmount::Zero, "Ch::" ; "extra hot coffee")]
#[test_case(Beverage::Tea(HotBeverageOption::ExtraHot), SugarAmount::Two, "Th:2:0" ; "extra hot tea")]
fn machine_dispenses_beverage(
    beverage: Beverage,
    sugar_amount: SugarAmount,
    expected_drink_maker_cmd: &str,
) {
    let drink_maker_spy = DrinkMakerSpy::new();
    let beverage_server = DrinkMakerBeverageServer::new(&drink_maker_spy);
    let beverage_quantity_checker_always_full = BeverageQuantityCheckerFake::new(false);
    let drink_maker_display = DrinkMakerDisplay::new(&drink_maker_spy);
    let mut machine = Machine::new(
        &beverage_server,
        &beverage_quantity_checker_always_full,
        &drink_maker_display,
        &DummyReportsPrinter {},
        &DummyNotifier {},
    );

    let beverage_request = BeverageRequest::new(&beverage, &sugar_amount, ENOUGH_MONEY);
    machine.dispense(beverage_request);

    let drink_maker_cmds = drink_maker_spy.get_received_commands();
    assert_eq!(
        drink_maker_cmds,
        vec![(String::from(expected_drink_maker_cmd))]
    )
}

#[test_case(Beverage::Coffee(HotBeverageOption::Standard), 1, "M:0.59€" ; "coffee, missing 0.59€")]
#[test_case(Beverage::Coffee(HotBeverageOption::ExtraHot), 59, "M:0.01€" ; "coffee, missing 0.01€")]
#[test_case(Beverage::Tea(HotBeverageOption::Standard), 1, "M:0.39€" ; "orane juice, missing 0.39€")]
#[test_case(Beverage::OrangeJuice, 59, "M:0.01€" ; "orane juice, missing 0.01€")]
fn machine_requires_money_to_dispense_beverage(
    beverage: Beverage,
    money_amount: u32,
    expected_drink_maker_cmd: &str,
) {
    let drink_maker_spy = DrinkMakerSpy::new();
    let beverage_server = DrinkMakerBeverageServer::new(&drink_maker_spy);
    let beverage_quantity_checker_always_full = BeverageQuantityCheckerFake::new(false);
    let drink_maker_display = DrinkMakerDisplay::new(&drink_maker_spy);
    let mut machine = Machine::new(
        &beverage_server,
        &beverage_quantity_checker_always_full,
        &drink_maker_display,
        &DummyReportsPrinter {},
        &DummyNotifier {},
    );

    let beverage_request = BeverageRequest::new(&beverage, &SugarAmount::Zero, money_amount);
    machine.dispense(beverage_request);

    let drink_maker_cmds = drink_maker_spy.get_received_commands();
    assert_eq!(
        drink_maker_cmds,
        vec![(String::from(expected_drink_maker_cmd))]
    )
}
