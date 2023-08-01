mod common;

use std::cell::RefCell;

use crate::common::{
    drink_maker_test_double::DrinkMakerTestDouble, dummy_notifier::DummyNotifier,
    dummy_reports_printer::DummyReportsPrinter,
};
use coffee_machine_kata_rust::{
    drink_maker::{
        drink_maker_beverage_server::DrinkMakerBeverageServer,
        drink_maker_display::DrinkMakerDisplay,
    },
    machine_system::{
        beverages::{
            beverage::Beverage, beverage::HotBeverageOption,
            beverage_quantity_checker::BeverageQuantityChecker, sugar_amount::SugarAmount, beverage_request::BeverageRequest,
        },
        machine_builder::MachineBuilder,
        notifier::Notifier,
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

pub(crate) struct NotifierTestDouble {
    missing_beverages_notifications: RefCell<Vec<Beverage>>,
}

impl NotifierTestDouble {
    pub(crate) fn new() -> NotifierTestDouble {
        NotifierTestDouble {
            missing_beverages_notifications: RefCell::new(Vec::new()),
        }
    }

    pub(crate) fn spied_missing_beverages_messages(&self) -> Vec<Beverage> {
        self.missing_beverages_notifications.borrow().clone()
    }
}

impl Notifier for NotifierTestDouble {
    fn notify_missing_beverage(&self, drink: &Beverage) {
        self.missing_beverages_notifications
            .borrow_mut()
            .push(drink.clone())
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
    let drink_maker_test_double = DrinkMakerTestDouble::new();
    let beverage_server = DrinkMakerBeverageServer::new(&drink_maker_test_double);
    let beverage_quantity_checker_fake_always_full = BeverageQuantityCheckerFake::new(false);
    let drink_maker_display = DrinkMakerDisplay::new(&drink_maker_test_double);
    let mut machine = MachineBuilder::new()
        .with_beverage_server(&beverage_server)
        .with_beverage_quantity_checker(&beverage_quantity_checker_fake_always_full)
        .with_display(&drink_maker_display)
        .with_reports_printer(&DummyReportsPrinter {})
        .with_notifier(&DummyNotifier {})
        .build();

    let beverage_request = BeverageRequest::new(&beverage, &sugar_amount, ENOUGH_MONEY);
    machine.dispense(beverage_request);

    let drink_maker_cmds = drink_maker_test_double.spied_received_commands();
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
    let drink_maker_test_double = DrinkMakerTestDouble::new();
    let beverage_server = DrinkMakerBeverageServer::new(&drink_maker_test_double);
    let beverage_quantity_checker_fake_always_full = BeverageQuantityCheckerFake::new(false);
    let drink_maker_display = DrinkMakerDisplay::new(&drink_maker_test_double);
    let mut machine = MachineBuilder::new()
        .with_beverage_server(&beverage_server)
        .with_beverage_quantity_checker(&beverage_quantity_checker_fake_always_full)
        .with_display(&drink_maker_display)
        .with_reports_printer(&DummyReportsPrinter {})
        .with_notifier(&DummyNotifier {})
        .build();

    let beverage_request = BeverageRequest::new(&beverage, &SugarAmount::Zero, money_amount);
    machine.dispense(beverage_request);

    let drink_maker_cmds = drink_maker_test_double.spied_received_commands();
    assert_eq!(
        drink_maker_cmds,
        vec![(String::from(expected_drink_maker_cmd))]
    )
}

#[test_case(Beverage::OrangeJuice, "M:Sorry, orange juice is empty." ; "orane juice empty")]
fn machine_handles_beverage_shortage(beverage: Beverage, expected_missing_beverage_message: &str) {
    let drink_maker_spy = DrinkMakerTestDouble::new();
    let beverage_server = DrinkMakerBeverageServer::new(&drink_maker_spy);
    let beverage_quantity_checker_fake_always_full = BeverageQuantityCheckerFake::new(true);
    let drink_maker_display = DrinkMakerDisplay::new(&drink_maker_spy);
    let notifier_test_double = NotifierTestDouble::new();
    let mut machine = MachineBuilder::new()
        .with_beverage_server(&beverage_server)
        .with_beverage_quantity_checker(&beverage_quantity_checker_fake_always_full)
        .with_display(&drink_maker_display)
        .with_reports_printer(&DummyReportsPrinter {})
        .with_notifier(&notifier_test_double)
        .build();

    let beverage_request = BeverageRequest::new(&beverage, &SugarAmount::Zero, ENOUGH_MONEY);
    machine.dispense(beverage_request);

    let drink_maker_cmds = drink_maker_spy.spied_received_commands();
    let missing_beverages_messages = notifier_test_double.spied_missing_beverages_messages();
    assert_eq!(
        drink_maker_cmds,
        vec![(String::from(expected_missing_beverage_message))]
    );
    assert_eq!(missing_beverages_messages, vec![Beverage::OrangeJuice]);
}
