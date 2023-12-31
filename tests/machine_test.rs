mod common;

use std::rc::Rc;

use crate::common::{
    drink_maker_test_double::DrinkMakerTestDouble, dummy_notifier::DummyNotifier,
    dummy_reports_printer::DummyReportsPrinter,
};
use crate::test_doubles::*;
use coffee_machine_kata_rust::prelude::DrinkMakerDisplay;
use coffee_machine_kata_rust::{
    drink_maker::drink_maker_beverage_server::DrinkMakerBeverageServer,
    machine_system::{
        beverages::{
            beverage::Beverage, beverage::HotBeverageOption, beverage_request::BeverageRequest,
            sugar_amount::SugarAmount,
        },
        machine_builder::MachineBuilder,
    },
};
use test_case::test_case;

mod test_doubles {
    use coffee_machine_kata_rust::prelude::{Beverage, BeverageQuantityChecker, Notifier};
    use std::cell::RefCell;

    pub(crate) const ENOUGH_MONEY: u32 = 100;

    pub(crate) struct BeverageQuantityCheckerFake {
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
        pub(crate) fn new() -> Self {
            Self {
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
}

#[test_case(Beverage::Coffee(HotBeverageOption::Standard), SugarAmount::One, "C:1:0" ; "coffee")]
#[test_case(Beverage::Coffee(HotBeverageOption::ExtraHot), SugarAmount::Zero, "Ch::" ; "extra hot coffee")]
#[test_case(Beverage::Tea(HotBeverageOption::ExtraHot), SugarAmount::Two, "Th:2:0" ; "extra hot tea")]
fn machine_dispenses_beverage(
    beverage: Beverage,
    sugar_amount: SugarAmount,
    expected_drink_maker_cmd: &str,
) {
    let drink_maker_test_double = Rc::new(DrinkMakerTestDouble::new());
    let beverage_server = Rc::new(DrinkMakerBeverageServer::new(Rc::clone(
        &drink_maker_test_double,
    )));
    let beverage_quantity_checker_fake_always_full =
        Rc::new(BeverageQuantityCheckerFake::new(false));
    let drink_maker_display = Rc::new(DrinkMakerDisplay::new(Rc::clone(&drink_maker_test_double)));
    let mut machine = MachineBuilder::default()
        .set(beverage_server)
        .set(beverage_quantity_checker_fake_always_full)
        .set(drink_maker_display)
        .set(Rc::new(DummyReportsPrinter {}))
        .set(Rc::new(DummyNotifier {}))
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
    let drink_maker_test_double = Rc::new(DrinkMakerTestDouble::new());
    let beverage_server = Rc::new(DrinkMakerBeverageServer::new(Rc::clone(
        &drink_maker_test_double,
    )));
    let beverage_quantity_checker_fake_always_full =
        Rc::new(BeverageQuantityCheckerFake::new(false));
    let drink_maker_display = Rc::new(DrinkMakerDisplay::new(Rc::clone(&drink_maker_test_double)));
    let mut machine = MachineBuilder::default()
        .set(beverage_server)
        .set(beverage_quantity_checker_fake_always_full)
        .set(drink_maker_display)
        .set(Rc::new(DummyReportsPrinter {}))
        .set(Rc::new(DummyNotifier {}))
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
    let drink_maker_spy = Rc::new(DrinkMakerTestDouble::new());
    let beverage_server = Rc::new(DrinkMakerBeverageServer::new(Rc::clone(&drink_maker_spy)));
    let beverage_quantity_checker_fake_always_full =
        Rc::new(BeverageQuantityCheckerFake::new(true));
    let drink_maker_display = Rc::new(DrinkMakerDisplay::new(Rc::clone(&drink_maker_spy)));
    let notifier_test_double = Rc::new(NotifierTestDouble::new());
    let mut machine = MachineBuilder::default()
        .set(beverage_server)
        .set(beverage_quantity_checker_fake_always_full)
        .set(drink_maker_display)
        .set(Rc::new(DummyReportsPrinter {}))
        .set(Rc::clone(&notifier_test_double))
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
