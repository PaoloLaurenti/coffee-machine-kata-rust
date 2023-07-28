use self::{
    beverage::Beverage,
    beverage_quantity_checker::BeverageQuantityChecker,
    beverage_request::BeverageRequest,
    beverage_server::BeverageServer,
    cashier::Cashier,
    dispenser::Dispenser,
    display::Display,
    notifier::Notifier,
    reports_printer::{PurchasesReport, ReportsPrinter},
    sugar_amount::SugarAmount,
};

pub mod beverage;
pub mod beverage_quantity_checker;
pub mod beverage_request;
pub mod beverage_server;
mod cashier;
mod dispenser;
pub mod display;
pub mod notifier;
pub mod reports_printer;
pub mod sugar_amount;

pub struct Machine<'a> {
    dispenser: Dispenser<'a>,
    cashier: Cashier,
    display: &'a dyn Display,
    reports_printer: &'a dyn ReportsPrinter,
    notifier: &'a dyn Notifier,
}

impl Machine<'_> {
    pub fn new<'a>(
        beverage_server: &'a impl BeverageServer,
        beverage_quantity_checker: &'a impl BeverageQuantityChecker,
        display: &'a impl Display,
        reports_printer: &'a impl ReportsPrinter,
        notifier: &'a impl Notifier,
    ) -> Machine<'a> {
        Machine {
            dispenser: Dispenser::new(beverage_server, beverage_quantity_checker),
            cashier: Cashier::new(),
            display,
            reports_printer,
            notifier,
        }
    }

    pub fn dispense2(&mut self, beverage_request: BeverageRequest) {
        let payment = self
            .cashier
            .checkout_payment(beverage_request.beverage, beverage_request.money_amount);

        match payment {
            cashier::BeveragePayment::Ok => {
                self.handle_dispense(beverage_request.beverage, beverage_request.sugar_amount)
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
            dispenser::BeverageDispsense::Shortage => self.handle_beverage_shortage(beverage),
        }
    }

    fn handle_beverage_shortage(&mut self, beverage: &Beverage) {
        self.cashier.refund_beverage_payment(beverage);
        self.notifier.notify_missing_beverage(beverage);
        self.display.show_beverage_shortage_message(beverage)
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

    struct ReportsPrinterTestDouble {
        reports_requested_to_print: RefCell<Vec<PurchasesReport>>,
    }

    impl ReportsPrinterTestDouble {
        fn new() -> ReportsPrinterTestDouble {
            ReportsPrinterTestDouble {
                reports_requested_to_print: RefCell::new(vec![]),
            }
        }

        fn spied_reports_requested_to_print(&self) -> Vec<PurchasesReport> {
            self.reports_requested_to_print.borrow().clone()
        }
    }

    impl ReportsPrinter for ReportsPrinterTestDouble {
        fn print(&self, purchase_report: PurchasesReport) {
            self.reports_requested_to_print
                .borrow_mut()
                .push(purchase_report.clone())
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

    struct BeverageServerTestDouble {
        requested_beverages: RefCell<Vec<(Beverage, SugarAmount)>>,
    }

    impl BeverageServerTestDouble {
        fn new() -> Self {
            Self {
                requested_beverages: RefCell::new(Vec::new()),
            }
        }

        fn spied_requested_beverages(&self) -> Vec<(Beverage, SugarAmount)> {
            self.requested_beverages.borrow().clone()
        }
    }

    impl BeverageServer for BeverageServerTestDouble {
        fn serve(&self, beverage: &Beverage, sugar_amount: &SugarAmount) {
            self.requested_beverages
                .borrow_mut()
                .push((beverage.clone(), sugar_amount.clone()));
        }
    }

    struct DummyBeverageServer {}
    impl BeverageServer for DummyBeverageServer {
        fn serve(&self, beverage: &Beverage, sugar_amount: &SugarAmount) {}
    }

    // struct DummyDispenser {
    //     dispensed_beverages_history: DispensedBeveragesHistory,
    // }

    // impl DummyDispenser {
    //     fn new() -> Self {
    //         Self {
    //             dispensed_beverages_history: DispensedBeveragesHistory::new(),
    //         }
    //     }
    // }

    // impl BeverageServer for DummyDispenser {
    //     fn serve(
    //         &mut self,
    //         _beverage: &Beverage,
    //         _sugar_amount: &SugarAmount,
    //     ) -> beverage_server::BeverageDispsense {
    //         BeverageDispsense::Ok
    //     }

    //     fn dispensed_beverages(&self) -> &DispensedBeveragesHistory {
    //         &self.dispensed_beverages_history
    //     }
    // }

    struct DisplayTestDouble {
        missing_money_message_requests: RefCell<Vec<u32>>,
        beverage_shortage_message_request: RefCell<Vec<Beverage>>,
    }

    impl DisplayTestDouble {
        fn new() -> Self {
            Self {
                missing_money_message_requests: RefCell::new(Vec::new()),
                beverage_shortage_message_request: RefCell::new(Vec::new()),
            }
        }

        fn spied_missing_money_message_requests(&self) -> Vec<u32> {
            self.missing_money_message_requests.borrow().clone()
        }

        fn spied_beverage_shortage_message_requests(&self) -> Vec<Beverage> {
            self.beverage_shortage_message_request.borrow().clone()
        }
    }

    impl Display for DisplayTestDouble {
        fn show_missing_money_message(&self, missing_money: u32) {
            self.missing_money_message_requests
                .borrow_mut()
                .push(missing_money);
        }

        fn show_beverage_shortage_message(&self, beverage: &Beverage) {
            self.beverage_shortage_message_request
                .borrow_mut()
                .push(beverage.clone());
        }
    }

    const ENOUGH_MONEY: u32 = 100;

    #[test_case(Beverage::Coffee(HotBeverageOption::Standard); "cofee")]
    #[test_case(Beverage::Coffee(HotBeverageOption::ExtraHot); "extra hot cofee")]
    #[test_case(Beverage::Tea(HotBeverageOption::Standard); "tea")]
    #[test_case(Beverage::Tea(HotBeverageOption::ExtraHot); "extra hot tea")]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::Standard); "hot chocolate")]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::ExtraHot) ; "extra hot hot chocolate")]
    #[test_case(Beverage::OrangeJuice; "Orange juice")]
    fn machine_dispenses_beverage_with_no_sugar(beverage: Beverage) {
        let mut beverage_server_test_double = BeverageServerTestDouble::new();
        let mut machine = Machine::new(
            &mut beverage_server_test_double,
            &StubInfiniteBeverageQuantityChecker {},
            &DummyDisplay {},
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );

        let beverage_request = BeverageRequest::new(&beverage, &SugarAmount::Zero, ENOUGH_MONEY);
        machine.dispense2(beverage_request);

        let requested_beverages = beverage_server_test_double.spied_requested_beverages();
        assert_eq!(
            requested_beverages,
            vec![(beverage.clone(), SugarAmount::Zero)]
        )
    }

    #[test_case(SugarAmount::One; "one sugar")]
    #[test_case(SugarAmount::Two; "two sugars")]
    fn machine_dispenses_beverage_with_sugar(sugar_amount: SugarAmount) {
        let mut beverage_server_test_double = BeverageServerTestDouble::new();
        let mut machine = Machine::new(
            &mut beverage_server_test_double,
            &StubInfiniteBeverageQuantityChecker {},
            &DummyDisplay {},
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );

        let beverage_request = BeverageRequest::new(
            &Beverage::Coffee(HotBeverageOption::Standard),
            &sugar_amount,
            ENOUGH_MONEY,
        );
        machine.dispense2(beverage_request);

        let requested_beverages = beverage_server_test_double.spied_requested_beverages();
        assert_eq!(
            requested_beverages,
            vec![(
                Beverage::Coffee(HotBeverageOption::Standard),
                sugar_amount.clone()
            )]
        )
    }

    #[test_case(Beverage::Coffee(HotBeverageOption::Standard), 60; "coffee costs 0.6€")]
    #[test_case(Beverage::Tea(HotBeverageOption::Standard), 40; "tea costs 0.4€")]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::Standard), 50; "hot chocolate costs 0.5€")]
    #[test_case(Beverage::OrangeJuice, 60; "orange juice costs 0.6€")]
    fn machine_dispenses_beverages_only_when_given_money_is_enough(
        beverage: Beverage,
        money_amount: u32,
    ) {
        let mut beverage_server_test_double = BeverageServerTestDouble::new();
        let mut machine = Machine::new(
            &mut beverage_server_test_double,
            &StubInfiniteBeverageQuantityChecker {},
            &DummyDisplay {},
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );

        let beverage_request = BeverageRequest::new(&beverage, &SugarAmount::Zero, money_amount);
        machine.dispense2(beverage_request);

        let requested_beverages = beverage_server_test_double.spied_requested_beverages();
        assert_eq!(
            requested_beverages,
            vec![(beverage.clone(), SugarAmount::Zero)]
        )
    }

    #[test_case(Beverage::Coffee(HotBeverageOption::Standard), 59; "coffee costs 0.6€")]
    #[test_case(Beverage::Tea(HotBeverageOption::Standard), 39; "tea costs 0.4€")]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::Standard), 49; "hot chocolate costs 0.5€")]
    #[test_case(Beverage::OrangeJuice, 59; "orange juice costs 0.6€")]
    fn machine_does_not_dispense_beverages_when_given_money_is_not_enough(
        beverage: Beverage,
        money_amount: u32,
    ) {
        let mut beverage_server_test_double = BeverageServerTestDouble::new();
        let mut machine = Machine::new(
            &mut beverage_server_test_double,
            &StubInfiniteBeverageQuantityChecker {},
            &DummyDisplay {},
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );

        let beverage_request = BeverageRequest::new(&beverage, &SugarAmount::Zero, money_amount);
        machine.dispense2(beverage_request);

        let requested_beverages = beverage_server_test_double.spied_requested_beverages();
        assert_eq!(requested_beverages, Vec::new())
    }

    #[test_case(Beverage::Coffee(HotBeverageOption::Standard), 59, 1; "coffee costs 0.6€, missing 0.01€")]
    #[test_case(Beverage::Coffee(HotBeverageOption::Standard), 1, 59; "coffee costs 0.6€, missing 0.59€")]
    #[test_case(Beverage::Tea(HotBeverageOption::Standard), 39, 1; "tea costs 0.4€, missing 0.01€")]
    #[test_case(Beverage::Tea(HotBeverageOption::Standard), 1, 39; "tea costs 0.4€, missing 0.39€")]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::Standard), 49, 1; "tea costs 0.5€, missing 0.01€")]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::Standard), 1, 49; "tea costs 0.5€, missing 0.49€")]
    #[test_case(Beverage::OrangeJuice, 59, 1; "orange juice costs 0.6€, missing 0.01€")]
    #[test_case(Beverage::OrangeJuice, 1, 59; "orange juice costs 0.6€, missing 0.59€")]
    fn machine_shows_missing_amount_when_asked_for_a_beverage_with_not_enough_money(
        beverage: Beverage,
        money_amount: u32,
        missing_money_amount: u32,
    ) {
        let display_test_double = DisplayTestDouble::new();
        let mut machine = Machine::new(
            &DummyBeverageServer {},
            &StubInfiniteBeverageQuantityChecker {},
            &display_test_double,
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );
        let beverage_request = BeverageRequest::new(&beverage, &SugarAmount::Zero, money_amount);
        machine.dispense2(beverage_request);

        let show_missing_money_message_requests =
            display_test_double.spied_missing_money_message_requests();
        assert_eq!(
            show_missing_money_message_requests,
            vec![missing_money_amount]
        )
    }

    #[test]
    fn machine_prints_purchases_report() {
        let reports_printer_test_double = ReportsPrinterTestDouble::new();
        let mut machine = Machine::new(
            &DummyBeverageServer {},
            &StubInfiniteBeverageQuantityChecker {},
            &DummyDisplay {},
            &reports_printer_test_double,
            &DummyNotifier {},
        );
        machine.dispense2(BeverageRequest::new(
            &Beverage::Coffee(HotBeverageOption::Standard),
            &SugarAmount::Zero,
            ENOUGH_MONEY,
        ));
        machine.dispense2(BeverageRequest::new(
            &Beverage::Coffee(HotBeverageOption::Standard),
            &SugarAmount::Zero,
            ENOUGH_MONEY,
        ));
        machine.dispense2(BeverageRequest::new(
            &Beverage::OrangeJuice,
            &SugarAmount::Zero,
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
            reports_printer_test_double.spied_reports_requested_to_print(),
            vec![expeted_report]
        )
    }

    #[test_case(Beverage::Coffee(HotBeverageOption::Standard))]
    #[test_case(Beverage::Tea(HotBeverageOption::Standard))]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::Standard))]
    #[test_case(Beverage::OrangeJuice)]
    fn machine_shows_shortage_message(beverage: Beverage) {
        let display_test_double = DisplayTestDouble::new();
        let mut machine = Machine::new(
            &DummyBeverageServer {},
            &StubEmptyBeverageQuantityChecker {},
            &display_test_double,
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );

        let beverage_request = BeverageRequest::new(&beverage, &SugarAmount::Zero, ENOUGH_MONEY);
        machine.dispense2(beverage_request);

        let beverage_shortage_message_requests =
            display_test_double.spied_beverage_shortage_message_requests();
        assert_eq!(beverage_shortage_message_requests, vec![beverage])
    }

    #[test]
    fn machine_does_not_dispense_the_requested_beverage_when_there_is_a_shortage() {
        let beverage_server_test_double = BeverageServerTestDouble::new();
        let mut machine = Machine::new(
            &beverage_server_test_double,
            &StubEmptyBeverageQuantityChecker {},
            &DummyDisplay {},
            &DummyReportsPrinter {},
            &DummyNotifier {},
        );

        let beverage_request = BeverageRequest::new(
            &Beverage::Coffee(HotBeverageOption::Standard),
            &SugarAmount::Zero,
            ENOUGH_MONEY,
        );
        machine.dispense2(beverage_request);

        let requested_beverages = beverage_server_test_double.spied_requested_beverages();
        assert_eq!(requested_beverages, Vec::new())
    }

    #[test]
    fn purchase_report_does_not_contain_beverages_not_dispensed_due_to_a_shortage() {
        let reports_printer_test_double = ReportsPrinterTestDouble::new();
        let stub_beverage_quantity_checker = StubBeverageQuantityChecker::new();
        stub_beverage_quantity_checker
            .stub_beverage_as_available(&Beverage::Coffee(HotBeverageOption::Standard));
        stub_beverage_quantity_checker.stub_beverage_as_empty(Beverage::OrangeJuice);
        let mut machine = Machine::new(
            &DummyBeverageServer {},
            &stub_beverage_quantity_checker,
            &DummyDisplay {},
            &reports_printer_test_double,
            &DummyNotifier {},
        );
        machine.dispense2(BeverageRequest::new(
            &Beverage::Coffee(HotBeverageOption::Standard),
            &SugarAmount::Zero,
            ENOUGH_MONEY,
        ));
        machine.dispense2(BeverageRequest::new(
            &Beverage::OrangeJuice,
            &SugarAmount::Zero,
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
            reports_printer_test_double.spied_reports_requested_to_print(),
            vec![expeted_report]
        )
    }

    #[test]
    fn machine_notifies_when_unable_to_dipsense_beverage_due_to_a_shortage() {
        let stub_beverage_quantity_checker = StubBeverageQuantityChecker::new();
        stub_beverage_quantity_checker
            .stub_beverage_as_available(&Beverage::Coffee(HotBeverageOption::Standard));
        stub_beverage_quantity_checker.stub_beverage_as_empty(Beverage::OrangeJuice);
        stub_beverage_quantity_checker
            .stub_beverage_as_empty(Beverage::Tea(HotBeverageOption::ExtraHot));
        let notifier_spy = NotifierSpy::new();
        let mut machine = Machine::new(
            &DummyBeverageServer {},
            &stub_beverage_quantity_checker,
            &DummyDisplay {},
            &DummyReportsPrinter {},
            &notifier_spy,
        );

        let coffee_beverage_request = BeverageRequest::new(
            &Beverage::Coffee(HotBeverageOption::Standard),
            &SugarAmount::Zero,
            ENOUGH_MONEY,
        );
        let orange_juice_beverage_request =
            BeverageRequest::new(&Beverage::OrangeJuice, &SugarAmount::Zero, ENOUGH_MONEY);
        let tea_beverage_request = BeverageRequest::new(
            &Beverage::Tea(HotBeverageOption::ExtraHot),
            &SugarAmount::Zero,
            ENOUGH_MONEY,
        );
        machine.dispense2(coffee_beverage_request);
        machine.dispense2(orange_juice_beverage_request);
        machine.dispense2(tea_beverage_request);

        let notified_missing_beverages = notifier_spy.missing_beverages_notifications();
        assert_eq!(
            notified_missing_beverages,
            vec![
                Beverage::OrangeJuice,
                Beverage::Tea(HotBeverageOption::ExtraHot)
            ]
        )
    }

    // tests review
    // machine mut dispenser , sure?
    // machine builder
}
