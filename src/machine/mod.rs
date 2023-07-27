use self::{
    beverage::Beverage,
    cashier::Cashier,
    dispenser::Dispenser,
    display::Display,
    notifier::Notifier,
    reports_printer::{PurchasesReport, ReportsPrinter},
    sugar_amount::SugarAmount,
};

pub mod beverage;
pub mod beverage_quantity_checker;
mod cashier;
pub mod dispenser;
pub mod display;
pub mod notifier;
pub mod reports_printer;
pub mod sugar_amount;

pub struct BeverageRequest {
    beverage: Beverage,
    sugar_amount: SugarAmount,
    money_amount: u32,
}

impl BeverageRequest {
    pub fn new(
        beverage: Beverage,
        sugar_amount: SugarAmount,
        money_amount: u32,
    ) -> BeverageRequest {
        BeverageRequest {
            beverage,
            sugar_amount,
            money_amount,
        }
    }
}

pub struct Machine<'a> {
    dispenser: &'a mut dyn Dispenser,
    cashier: Cashier,
    display: &'a dyn Display,
    reports_printer: &'a dyn ReportsPrinter,
    notifier: &'a dyn Notifier,
}

impl Machine<'_> {
    pub fn new<'a>(
        dispenser: &'a mut impl Dispenser,
        display: &'a impl Display,
        reports_printer: &'a impl ReportsPrinter,
        notifier: &'a impl Notifier,
    ) -> Machine<'a> {
        Machine {
            dispenser,
            cashier: Cashier::new(),
            display,
            reports_printer,
            notifier,
        }
    }

    pub fn dispense(&mut self, beverage_request: BeverageRequest) {
        let payment = self
            .cashier
            .checkout_payment(&beverage_request.beverage, beverage_request.money_amount);

        match payment {
            cashier::BeveragePayment::Ok => {
                self.handle_dispense(&beverage_request.beverage, &beverage_request.sugar_amount)
            }
            cashier::BeveragePayment::NotEnoughMoney(missing_money_amount) => self
                .display
                .show_missing_money_message(missing_money_amount),
        }
    }

    fn handle_dispense(&mut self, beverage: &Beverage, sugar_amount: &SugarAmount) {
        let dispensed = self.dispenser.dispense(beverage, sugar_amount);

        match dispensed {
            dispenser::BeverageDispsense::Ok => (),
            dispenser::BeverageDispsense::Shortage => {
                self.cashier.refund_beverage_payment(beverage);
                self.notifier.notify_missing_beverage(beverage);
                self.display.show_beverage_shortage_message(beverage)
            }
        }
    }

    pub fn print_purchases_report(&self) {
        let dispensed_beverages_history = self.dispenser.dispensed_beverages();
        let total_money_earned = self.cashier.total_money_earned();

        let purchase_report =
            PurchasesReport::new(&dispensed_beverages_history.quantities, total_money_earned);
        self.reports_printer.print(purchase_report)
    }
}

#[cfg(test)]
mod machine_tests {
    use crate::drink_maker::drink_maker_dispenser::DrinkMakerDispenser;
    use crate::drink_maker::drink_maker_display::DrinkMakerDisplay;
    use crate::drink_maker::DrinkMaker;
    use crate::machine::beverage::HotBeverageOption;
    use crate::machine::reports_printer::{PurchasesReport, ReportsPrinter};

    use super::beverage_quantity_checker::BeverageQuantityChecker;
    use super::notifier::Notifier;
    use super::*;
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use test_case::test_case;

    struct DrinkMakerSpy {
        received_commands: RefCell<Vec<String>>,
    }

    impl DrinkMakerSpy {
        fn new() -> DrinkMakerSpy {
            DrinkMakerSpy {
                received_commands: RefCell::new(vec![]),
            }
        }

        pub fn get_received_commands(&self) -> Vec<String> {
            self.received_commands.clone().take()
        }
    }

    impl DrinkMaker for DrinkMakerSpy {
        fn execute(&self, command: String) {
            self.received_commands.borrow_mut().push(command);
        }
    }

    struct DummyDrinkMaker {}

    impl DrinkMaker for DummyDrinkMaker {
        fn execute(&self, _command: String) {}
    }

    struct DummyReportsPrinter {}
    impl ReportsPrinter for DummyReportsPrinter {
        fn print(&self, _purchase_report: PurchasesReport) {}
    }

    struct DummyDisplay {}
    impl Display for DummyDisplay {
        fn show_missing_money_message(&self, _missing_money: u32) {}

        fn show_beverage_shortage_message(&self, _beverage: &Beverage) {}
    }

    struct StubInfiniteBeverageQuantityChecker {}
    impl BeverageQuantityChecker for StubInfiniteBeverageQuantityChecker {
        fn is_empty(&self, _beverage: &Beverage) -> bool {
            false
        }
    }

    struct StubEmptyBeverageQuantityChecker {}
    impl BeverageQuantityChecker for StubEmptyBeverageQuantityChecker {
        fn is_empty(&self, _beverage: &Beverage) -> bool {
            true
        }
    }

    struct StubBeverageQuantityChecker {
        empty_beverages: RefCell<HashSet<Beverage>>,
    }
    impl StubBeverageQuantityChecker {
        fn new() -> Self {
            StubBeverageQuantityChecker {
                empty_beverages: RefCell::new(HashSet::new()),
            }
        }

        fn stub_beverage_as_available(&self, _beverage: &Beverage) {}

        fn stub_beverage_as_empty(&self, beverage: Beverage) {
            self.empty_beverages.borrow_mut().insert(beverage);
        }
    }
    impl BeverageQuantityChecker for StubBeverageQuantityChecker {
        fn is_empty(&self, beverage: &Beverage) -> bool {
            self.empty_beverages.borrow().contains(beverage)
        }
    }

    struct ReportsPrinterSpy {
        reports_requested_to_print: RefCell<Vec<PurchasesReport>>,
    }

    impl ReportsPrinterSpy {
        fn new() -> ReportsPrinterSpy {
            ReportsPrinterSpy {
                reports_requested_to_print: RefCell::new(vec![]),
            }
        }
    }

    impl ReportsPrinter for ReportsPrinterSpy {
        fn print(&self, purchase_report: PurchasesReport) {
            self.reports_requested_to_print
                .borrow_mut()
                .push(purchase_report)
        }
    }

    struct NotifierSpy {
        missing_beverages_notifications: RefCell<Vec<Beverage>>,
    }

    impl NotifierSpy {
        fn new() -> NotifierSpy {
            NotifierSpy {
                missing_beverages_notifications: RefCell::new(Vec::new()),
            }
        }

        fn missing_beverages_notifications(&self) -> Vec<Beverage> {
            self.missing_beverages_notifications.borrow().clone()
        }
    }

    impl Notifier for NotifierSpy {
        fn notify_missing_beverage(&self, drink: &Beverage) {
            self.missing_beverages_notifications
                .borrow_mut()
                .push(drink.clone())
        }
    }

    struct DummyNotifier {}
    impl Notifier for DummyNotifier {
        fn notify_missing_beverage(&self, _drink: &Beverage) {}
    }

    const ENOUGH_MONEY: u32 = 100;

    #[test_case(Beverage::Coffee(HotBeverageOption::Standard), "C::" ; "cofee")]
    #[test_case(Beverage::Coffee(HotBeverageOption::ExtraHot), "Ch::" ; "extra hot cofee")]
    #[test_case(Beverage::Tea(HotBeverageOption::Standard), "T::" ; "tea")]
    #[test_case(Beverage::Tea(HotBeverageOption::ExtraHot), "Th::" ; "extra hot tea")]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::Standard), "H::" ; "hot chocolate")]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::ExtraHot), "Hh::" ; "extra hot hot chocolate")]
    #[test_case(Beverage::OrangeJuice, "O::" ; "Orange juice")]
    fn machine_dispenses_beverage_with_no_sugar_no_stick(
        beverage: Beverage,
        expected_drink_maker_cmd: &str,
    ) {
        let drink_maker_spy = DrinkMakerSpy::new();
        let mut dispenser =
            DrinkMakerDispenser::new(&drink_maker_spy, &StubInfiniteBeverageQuantityChecker {});
        let display = DrinkMakerDisplay::new(&drink_maker_spy);
        let mut machine = Machine::new(
            &mut dispenser,
            &display,
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );

        let beverage_request = BeverageRequest::new(beverage, SugarAmount::Zero, ENOUGH_MONEY);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(
            drink_maker_cmds,
            vec![String::from(expected_drink_maker_cmd)]
        )
    }

    #[test_case(SugarAmount::One, "1" ; "one sugar")]
    #[test_case(SugarAmount::Two, "2" ; "two sugars")]
    fn machine_dispenses_beverage_with_one_sugar(
        sugar_amount: SugarAmount,
        expected_sugar_amount_cmd_part: &str,
    ) {
        let drink_maker_spy = DrinkMakerSpy::new();
        let mut dispenser =
            DrinkMakerDispenser::new(&drink_maker_spy, &StubInfiniteBeverageQuantityChecker {});
        let display = DrinkMakerDisplay::new(&drink_maker_spy);
        let mut machine = Machine::new(
            &mut dispenser,
            &display,
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );

        let beverage_request = BeverageRequest::new(
            Beverage::Coffee(HotBeverageOption::Standard),
            sugar_amount,
            ENOUGH_MONEY,
        );
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(1, drink_maker_cmds.len());
        let sugar_amount_cmd_part = drink_maker_cmds[0].split(':').nth(1).unwrap();
        assert_eq!(sugar_amount_cmd_part, expected_sugar_amount_cmd_part)
    }

    #[test_case(SugarAmount::One, "0" ; "stick with one sugar")]
    #[test_case(SugarAmount::Two, "0" ; "stick with two sugars")]
    fn machine_dispenses_beverage_with_stick_when_some_sugar_is_requested(
        sugar_amount: SugarAmount,
        expected_stick_cmd_part: &str,
    ) {
        let drink_maker_spy = DrinkMakerSpy::new();
        let mut dispenser =
            DrinkMakerDispenser::new(&drink_maker_spy, &StubInfiniteBeverageQuantityChecker {});
        let display = DrinkMakerDisplay::new(&drink_maker_spy);
        let mut machine = Machine::new(
            &mut dispenser,
            &display,
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );

        let beverage_request = BeverageRequest::new(
            Beverage::Coffee(HotBeverageOption::Standard),
            sugar_amount,
            ENOUGH_MONEY,
        );
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(1, drink_maker_cmds.len());
        let stick_cmd_part = drink_maker_cmds[0].split(':').nth(2).unwrap();
        assert_eq!(stick_cmd_part, expected_stick_cmd_part)
    }

    #[test_case(Beverage::Coffee(HotBeverageOption::Standard), 60, "C::" ; "coffee costs 0.6€")]
    #[test_case(Beverage::Tea(HotBeverageOption::Standard), 40, "T::" ; "tea costs 0.4€")]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::Standard), 50, "H::" ; "hot chocolate costs 0.5€")]
    #[test_case(Beverage::OrangeJuice, 60, "O::" ; "orange juice costs 0.6€")]
    fn machine_dispenses_beverages_only_when_given_money_is_enough(
        beverage: Beverage,
        money_amount: u32,
        expected_drink_maker_cmd: &str,
    ) {
        let drink_maker_spy = DrinkMakerSpy::new();
        let mut dispenser =
            DrinkMakerDispenser::new(&drink_maker_spy, &StubInfiniteBeverageQuantityChecker {});
        let display = DrinkMakerDisplay::new(&drink_maker_spy);
        let mut machine = Machine::new(
            &mut dispenser,
            &display,
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );

        let beverage_request = BeverageRequest::new(beverage, SugarAmount::Zero, money_amount);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(
            drink_maker_cmds,
            vec![String::from(expected_drink_maker_cmd)]
        )
    }

    #[test_case(Beverage::Coffee(HotBeverageOption::Standard), 59, "C::"; "coffee costs 0.6€")]
    #[test_case(Beverage::Tea(HotBeverageOption::Standard), 39, "T::" ; "tea costs 0.4€")]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::Standard), 49, "H::" ; "hot chocolate costs 0.5€")]
    #[test_case(Beverage::OrangeJuice, 59, "O::" ; "orange juice costs 0.6€")]
    fn machine_does_not_dispense_beverages_when_given_money_is_not_enough(
        beverage: Beverage,
        money_amount: u32,
        dispense_drink_maker_cmd: &str,
    ) {
        let drink_maker_spy = DrinkMakerSpy::new();
        let mut dispenser =
            DrinkMakerDispenser::new(&drink_maker_spy, &StubInfiniteBeverageQuantityChecker {});
        let display = DrinkMakerDisplay::new(&drink_maker_spy);
        let mut machine = Machine::new(
            &mut dispenser,
            &display,
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );

        let beverage_request = BeverageRequest::new(beverage, SugarAmount::Zero, money_amount);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert!(!drink_maker_cmds.contains(&dispense_drink_maker_cmd.to_string()))
    }

    #[test_case(Beverage::Coffee(HotBeverageOption::Standard), 59, "M:0.01€"; "coffee costs 0.6€, missing 0.01€")]
    #[test_case(Beverage::Coffee(HotBeverageOption::Standard), 1, "M:0.59€"; "coffee costs 0.6€, missing 0.59€")]
    #[test_case(Beverage::Tea(HotBeverageOption::Standard), 39, "M:0.01€"; "tea costs 0.4€, missing 0.01€")]
    #[test_case(Beverage::Tea(HotBeverageOption::Standard), 1, "M:0.39€"; "tea costs 0.4€, missing 0.39€")]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::Standard), 49, "M:0.01€"; "tea costs 0.5€, missing 0.01€")]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::Standard), 1, "M:0.49€"; "tea costs 0.5€, missing 0.49€")]
    #[test_case(Beverage::OrangeJuice, 59, "M:0.01€"; "orange juice costs 0.6€, missing 0.01€")]
    #[test_case(Beverage::OrangeJuice, 1, "M:0.59€"; "orange juice costs 0.6€, missing 0.59€")]
    fn machine_shows_missing_amount_when_asked_for_a_beverage_with_not_enough_money(
        beverage: Beverage,
        money_amount: u32,
        expected_drink_maker_cmd: &str,
    ) {
        let drink_maker_spy = DrinkMakerSpy::new();
        let mut dispenser =
            DrinkMakerDispenser::new(&drink_maker_spy, &StubInfiniteBeverageQuantityChecker {});
        let display = DrinkMakerDisplay::new(&drink_maker_spy);
        let mut machine = Machine::new(
            &mut dispenser,
            &display,
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );

        let beverage_request = BeverageRequest::new(beverage, SugarAmount::Zero, money_amount);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(
            drink_maker_cmds,
            vec![String::from(expected_drink_maker_cmd)]
        )
    }

    #[test]
    fn machine_prints_purchases_report() {
        let dummy_drink_maker = DummyDrinkMaker {};
        let reports_printer_spy = ReportsPrinterSpy::new();
        let mut dispenser =
            DrinkMakerDispenser::new(&dummy_drink_maker, &StubInfiniteBeverageQuantityChecker {});
        let mut machine = Machine::new(
            &mut dispenser,
            &(DummyDisplay {}),
            &reports_printer_spy,
            &DummyNotifier {},
        );
        machine.dispense(BeverageRequest::new(
            Beverage::Coffee(HotBeverageOption::Standard),
            SugarAmount::Zero,
            ENOUGH_MONEY,
        ));
        machine.dispense(BeverageRequest::new(
            Beverage::Coffee(HotBeverageOption::Standard),
            SugarAmount::Zero,
            ENOUGH_MONEY,
        ));
        machine.dispense(BeverageRequest::new(
            Beverage::OrangeJuice,
            SugarAmount::Zero,
            ENOUGH_MONEY,
        ));

        machine.print_purchases_report();

        let mut beverages: HashMap<Beverage, u32> = HashMap::new();
        beverages.insert(Beverage::Coffee(HotBeverageOption::Standard), 2);
        beverages.insert(Beverage::OrangeJuice, 1);
        let expeted_report = PurchasesReport {
            beverages_quantities: beverages,
            total_money_earned: 180,
        };
        assert_eq!(
            reports_printer_spy.reports_requested_to_print.take(),
            vec![expeted_report]
        )
    }

    #[test_case(
        Beverage::Coffee(HotBeverageOption::Standard),
        "M:Sorry, coffee is empty."
    )]
    #[test_case(Beverage::Tea(HotBeverageOption::Standard), "M:Sorry, tea is empty.")]
    #[test_case(
        Beverage::HotChocolate(HotBeverageOption::Standard),
        "M:Sorry, hot chocolate is empty."
    )]
    #[test_case(Beverage::OrangeJuice, "M:Sorry, orange juice is empty.")]
    fn machine_shows_shortage_message(beverage: Beverage, exptected_shortage_message: &str) {
        let drink_maker_spy = DrinkMakerSpy::new();
        let mut dispenser =
            DrinkMakerDispenser::new(&drink_maker_spy, &StubEmptyBeverageQuantityChecker {});
        let display = DrinkMakerDisplay::new(&drink_maker_spy);
        let mut machine = Machine::new(
            &mut dispenser,
            &display,
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );

        let beverage_request = BeverageRequest::new(beverage, SugarAmount::Zero, ENOUGH_MONEY);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert!(
            drink_maker_cmds.contains(&exptected_shortage_message.to_string()),
            "No coffee shortage message command has been requested to the drink maker. Commands received are {:?}", drink_maker_cmds
        )
    }

    #[test]
    fn machine_does_not_dispense_the_requested_beverage_when_there_is_a_shortage() {
        let drink_maker_spy = DrinkMakerSpy::new();
        let mut dispenser =
            DrinkMakerDispenser::new(&drink_maker_spy, &StubEmptyBeverageQuantityChecker {});
        let display = DrinkMakerDisplay::new(&drink_maker_spy);
        let mut machine = Machine::new(
            &mut dispenser,
            &display,
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );

        let beverage_request = BeverageRequest::new(
            Beverage::Coffee(HotBeverageOption::Standard),
            SugarAmount::Zero,
            ENOUGH_MONEY,
        );
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert!(
            !drink_maker_cmds.contains(&"C::".to_string()),
            "Request to dispense a coffee has been sent to the drink maker even it there's a shortage. Commands received are {:?}", drink_maker_cmds
        )
    }

    #[test]
    fn purchase_report_does_not_contain_beverages_not_dispensed_due_to_a_shortage() {
        let dummy_drink_maker = DummyDrinkMaker {};
        let reports_printer_spy = ReportsPrinterSpy::new();
        let stub_beverage_quantity_checker = StubBeverageQuantityChecker::new();
        stub_beverage_quantity_checker
            .stub_beverage_as_available(&Beverage::Coffee(HotBeverageOption::Standard));
        stub_beverage_quantity_checker.stub_beverage_as_empty(Beverage::OrangeJuice);
        let mut dispenser =
            DrinkMakerDispenser::new(&dummy_drink_maker, &stub_beverage_quantity_checker);
        let mut machine = Machine::new(
            &mut dispenser,
            &(DummyDisplay {}),
            &reports_printer_spy,
            &DummyNotifier {},
        );
        machine.dispense(BeverageRequest::new(
            Beverage::Coffee(HotBeverageOption::Standard),
            SugarAmount::Zero,
            ENOUGH_MONEY,
        ));
        machine.dispense(BeverageRequest::new(
            Beverage::OrangeJuice,
            SugarAmount::Zero,
            ENOUGH_MONEY,
        ));

        machine.print_purchases_report();

        let mut beverages: HashMap<Beverage, u32> = HashMap::new();
        beverages.insert(Beverage::Coffee(HotBeverageOption::Standard), 1);
        let expeted_report = PurchasesReport {
            beverages_quantities: beverages,
            total_money_earned: 60,
        };
        assert_eq!(
            reports_printer_spy.reports_requested_to_print.take(),
            vec![expeted_report]
        )
    }

    #[test]
    fn machine_notifies_when_unable_to_dipsense_beverage_due_to_a_shortage() {
        let notifier_spy = NotifierSpy::new();
        let dummy_drink_maker = DummyDrinkMaker {};
        let stub_beverage_quantity_checker = StubBeverageQuantityChecker::new();
        stub_beverage_quantity_checker
            .stub_beverage_as_available(&Beverage::Coffee(HotBeverageOption::Standard));
        stub_beverage_quantity_checker.stub_beverage_as_empty(Beverage::OrangeJuice);
        stub_beverage_quantity_checker
            .stub_beverage_as_empty(Beverage::Tea(HotBeverageOption::ExtraHot));
        let mut dispenser =
            DrinkMakerDispenser::new(&dummy_drink_maker, &stub_beverage_quantity_checker);
        let mut machine = Machine::new(
            &mut dispenser,
            &(DummyDisplay {}),
            &DummyReportsPrinter {},
            &notifier_spy,
        );

        let coffee_beverage_request = BeverageRequest::new(
            Beverage::Coffee(HotBeverageOption::Standard),
            SugarAmount::Zero,
            ENOUGH_MONEY,
        );
        let orange_juice_beverage_request =
            BeverageRequest::new(Beverage::OrangeJuice, SugarAmount::Zero, ENOUGH_MONEY);
        let tea_beverage_request = BeverageRequest::new(
            Beverage::Tea(HotBeverageOption::ExtraHot),
            SugarAmount::Zero,
            ENOUGH_MONEY,
        );
        machine.dispense(coffee_beverage_request);
        machine.dispense(orange_juice_beverage_request);
        machine.dispense(tea_beverage_request);

        let notified_missing_beverages = notifier_spy.missing_beverages_notifications();
        assert_eq!(
            notified_missing_beverages,
            vec![
                Beverage::OrangeJuice,
                Beverage::Tea(HotBeverageOption::ExtraHot)
            ]
        )
    }
}
