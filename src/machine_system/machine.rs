use std::rc::Rc;

use super::{
    beverages::{
        beverage::Beverage,
        beverage_request::BeverageRequest,
        dispenser::{self, Dispenser},
        sugar_amount::SugarAmount,
    },
    cashier::{self, Cashier},
    display::Display,
    notifier::Notifier,
    reports_printer::{PurchasesReport, ReportsPrinter},
};

pub struct Machine {
    pub(crate) dispenser: Dispenser,
    pub(crate) cashier: Cashier,
    pub(crate) display: Rc<dyn Display>,
    pub(crate) reports_printer: Rc<dyn ReportsPrinter>,
    pub(crate) notifier: Rc<dyn Notifier>,
}

impl Machine {
    pub fn dispense(&mut self, beverage_request: BeverageRequest) {
        let payment = self
            .cashier
            .checkout_payment(beverage_request.beverage, beverage_request.money_amount);

        match payment {
            Ok(_) => self.handle_dispense(beverage_request.beverage, beverage_request.sugar_amount),
            Err(cashier::UnsuccessfulPayment::NotEnoughMoney(missing_money_amount)) => self
                .display
                .show_missing_money_message(missing_money_amount),
        }
    }

    fn handle_dispense(&mut self, beverage: &Beverage, sugar_amount: &SugarAmount) {
        let dispensed = self.dispenser.dispense(beverage, sugar_amount);

        if let dispenser::BeverageDispsense::Shortage = dispensed {
            self.handle_beverage_shortage(beverage)
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
pub(crate) mod machine_tests {
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::rc::Rc;
    use test_case::test_case;

    use crate::machine_system::beverages::{
        beverage::Beverage, beverage::HotBeverageOption,
        beverage_quantity_checker::BeverageQuantityChecker, beverage_request::BeverageRequest,
        beverage_server::BeverageServer, sugar_amount::SugarAmount,
    };
    use crate::machine_system::display::Display;
    use crate::machine_system::machine_builder::MachineBuilder;
    use crate::machine_system::notifier::Notifier;
    use crate::machine_system::reports_printer::{PurchasesReport, ReportsPrinter};

    const ENOUGH_MONEY: u32 = 100;

    pub(crate) struct DummyBeverageServer {}
    impl BeverageServer for DummyBeverageServer {
        fn serve(&self, _beverage: &Beverage, _sugar_amount: &SugarAmount) {}
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

    pub(crate) struct InfiniteBeverageQuantityCheckerFake {}
    impl BeverageQuantityChecker for InfiniteBeverageQuantityCheckerFake {
        fn is_empty(&self, _beverage: &Beverage) -> bool {
            false
        }
    }

    struct EmptyBeverageQuantityCheckerFake {}
    impl BeverageQuantityChecker for EmptyBeverageQuantityCheckerFake {
        fn is_empty(&self, _beverage: &Beverage) -> bool {
            true
        }
    }

    struct BeverageQuantityCheckerTestDouble {
        empty_beverages: RefCell<HashSet<Beverage>>,
    }

    impl BeverageQuantityCheckerTestDouble {
        fn new() -> Self {
            BeverageQuantityCheckerTestDouble {
                empty_beverages: RefCell::new(HashSet::new()),
            }
        }

        fn stub_beverage_as_available(&self, _beverage: &Beverage) {}

        fn stub_beverage_as_empty(&self, beverage: Beverage) {
            self.empty_beverages.borrow_mut().insert(beverage);
        }
    }

    impl BeverageQuantityChecker for BeverageQuantityCheckerTestDouble {
        fn is_empty(&self, beverage: &Beverage) -> bool {
            self.empty_beverages.borrow().contains(beverage)
        }
    }

    pub(crate) struct DummyDisplay {}
    impl Display for DummyDisplay {
        fn show_missing_money_message(&self, _missing_money: u32) {}

        fn show_beverage_shortage_message(&self, _beverage: &Beverage) {}
    }

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

    pub(crate) struct DummyReportsPrinter {}
    impl ReportsPrinter for DummyReportsPrinter {
        fn print(&self, _purchase_report: PurchasesReport) {}
    }

    struct ReportsPrinterTestDouble {
        reports_requested_to_print: RefCell<Vec<PurchasesReport>>,
    }

    impl ReportsPrinterTestDouble {
        fn new() -> Self {
            Self {
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

    pub(crate) struct DummyNotifier {}
    impl Notifier for DummyNotifier {
        fn notify_missing_beverage(&self, _drink: &Beverage) {}
    }

    struct NotifierTestDouble {
        missing_beverages_notifications: RefCell<Vec<Beverage>>,
    }

    impl NotifierTestDouble {
        fn new() -> Self {
            Self {
                missing_beverages_notifications: RefCell::new(Vec::new()),
            }
        }

        fn spied_missing_beverages_notifications(&self) -> Vec<Beverage> {
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

    #[test_case(Beverage::Coffee(HotBeverageOption::Standard); "cofee")]
    #[test_case(Beverage::Coffee(HotBeverageOption::ExtraHot); "extra hot cofee")]
    #[test_case(Beverage::Tea(HotBeverageOption::Standard); "tea")]
    #[test_case(Beverage::Tea(HotBeverageOption::ExtraHot); "extra hot tea")]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::Standard); "hot chocolate")]
    #[test_case(Beverage::HotChocolate(HotBeverageOption::ExtraHot) ; "extra hot hot chocolate")]
    #[test_case(Beverage::OrangeJuice; "Orange juice")]
    fn machine_dispenses_beverage_with_no_sugar(beverage: Beverage) {
        let beverage_server_test_double = Rc::new(BeverageServerTestDouble::new());
        let mut machine = MachineBuilder::default()
            .set(Rc::clone(&beverage_server_test_double))
            .set(Rc::new(InfiniteBeverageQuantityCheckerFake {}))
            .set(Rc::new(DummyDisplay {}))
            .set(Rc::new(DummyReportsPrinter {}))
            .set(Rc::new(DummyNotifier {}))
            .build();

        let beverage_request = BeverageRequest::new(&beverage, &SugarAmount::Zero, ENOUGH_MONEY);
        machine.dispense(beverage_request);

        let requested_beverages = beverage_server_test_double.spied_requested_beverages();
        assert_eq!(
            requested_beverages,
            vec![(beverage.clone(), SugarAmount::Zero)]
        )
    }

    #[test_case(SugarAmount::One; "one sugar")]
    #[test_case(SugarAmount::Two; "two sugars")]
    fn machine_dispenses_beverage_with_sugar(sugar_amount: SugarAmount) {
        let beverage_server_test_double = Rc::new(BeverageServerTestDouble::new());
        let mut machine = MachineBuilder::default()
            .set(Rc::clone(&beverage_server_test_double))
            .set(Rc::new(InfiniteBeverageQuantityCheckerFake {}))
            .set(Rc::new(DummyDisplay {}))
            .set(Rc::new(DummyReportsPrinter {}))
            .set(Rc::new(DummyNotifier {}))
            .build();

        let beverage_request = BeverageRequest::new(
            &Beverage::Coffee(HotBeverageOption::Standard),
            &sugar_amount,
            ENOUGH_MONEY,
        );
        machine.dispense(beverage_request);

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
        let beverage_server_test_double = Rc::new(BeverageServerTestDouble::new());
        let mut machine = MachineBuilder::default()
            .set(Rc::clone(&beverage_server_test_double))
            .set(Rc::new(InfiniteBeverageQuantityCheckerFake {}))
            .set(Rc::new(DummyDisplay {}))
            .set(Rc::new(DummyReportsPrinter {}))
            .set(Rc::new(DummyNotifier {}))
            .build();

        let beverage_request = BeverageRequest::new(&beverage, &SugarAmount::Zero, money_amount);
        machine.dispense(beverage_request);

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
        let beverage_server_test_double = Rc::new(BeverageServerTestDouble::new());
        let mut machine = MachineBuilder::default()
            .set(Rc::clone(&beverage_server_test_double))
            .set(Rc::new(InfiniteBeverageQuantityCheckerFake {}))
            .set(Rc::new(DummyDisplay {}))
            .set(Rc::new(DummyReportsPrinter {}))
            .set(Rc::new(DummyNotifier {}))
            .build();

        let beverage_request = BeverageRequest::new(&beverage, &SugarAmount::Zero, money_amount);
        machine.dispense(beverage_request);

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
        let display_test_double = Rc::new(DisplayTestDouble::new());
        let mut machine = MachineBuilder::default()
            .set(Rc::new(DummyBeverageServer {}))
            .set(Rc::new(InfiniteBeverageQuantityCheckerFake {}))
            .set(Rc::clone(&display_test_double))
            .set(Rc::new(DummyReportsPrinter {}))
            .set(Rc::new(DummyNotifier {}))
            .build();

        let beverage_request = BeverageRequest::new(&beverage, &SugarAmount::Zero, money_amount);
        machine.dispense(beverage_request);

        let show_missing_money_message_requests =
            display_test_double.spied_missing_money_message_requests();
        assert_eq!(
            show_missing_money_message_requests,
            vec![missing_money_amount]
        )
    }

    #[test]
    fn machine_prints_purchases_report() {
        let reports_printer_test_double = Rc::new(ReportsPrinterTestDouble::new());
        let mut machine = MachineBuilder::default()
            .set(Rc::new(DummyBeverageServer {}))
            .set(Rc::new(InfiniteBeverageQuantityCheckerFake {}))
            .set(Rc::new(DummyDisplay {}))
            .set(Rc::clone(&reports_printer_test_double))
            .set(Rc::new(DummyNotifier {}))
            .build();
        machine.dispense(BeverageRequest::new(
            &Beverage::Coffee(HotBeverageOption::Standard),
            &SugarAmount::Zero,
            ENOUGH_MONEY,
        ));
        machine.dispense(BeverageRequest::new(
            &Beverage::Coffee(HotBeverageOption::Standard),
            &SugarAmount::Zero,
            ENOUGH_MONEY,
        ));
        machine.dispense(BeverageRequest::new(
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
        let display_test_double = Rc::new(DisplayTestDouble::new());
        let mut machine = MachineBuilder::default()
            .set(Rc::new(DummyBeverageServer {}))
            .set(Rc::new(EmptyBeverageQuantityCheckerFake {}))
            .set(Rc::clone(&display_test_double))
            .set(Rc::new(DummyReportsPrinter {}))
            .set(Rc::new(DummyNotifier {}))
            .build();

        let beverage_request = BeverageRequest::new(&beverage, &SugarAmount::Zero, ENOUGH_MONEY);
        machine.dispense(beverage_request);

        let beverage_shortage_message_requests =
            display_test_double.spied_beverage_shortage_message_requests();
        assert_eq!(beverage_shortage_message_requests, vec![beverage])
    }

    #[test]
    fn machine_does_not_dispense_the_requested_beverage_when_there_is_a_shortage() {
        let beverage_server_test_double = Rc::new(BeverageServerTestDouble::new());
        let mut machine = MachineBuilder::default()
            .set(Rc::clone(&beverage_server_test_double))
            .set(Rc::new(EmptyBeverageQuantityCheckerFake {}))
            .set(Rc::new(DummyDisplay {}))
            .set(Rc::new(DummyReportsPrinter {}))
            .set(Rc::new(DummyNotifier {}))
            .build();

        let beverage_request = BeverageRequest::new(
            &Beverage::Coffee(HotBeverageOption::Standard),
            &SugarAmount::Zero,
            ENOUGH_MONEY,
        );
        machine.dispense(beverage_request);

        let requested_beverages = beverage_server_test_double.spied_requested_beverages();
        assert_eq!(requested_beverages, Vec::new())
    }

    #[test]
    fn purchase_report_does_not_contain_beverages_not_dispensed_due_to_a_shortage() {
        let reports_printer_test_double = Rc::new(ReportsPrinterTestDouble::new());
        let stub_beverage_quantity_checker = Rc::new(BeverageQuantityCheckerTestDouble::new());
        stub_beverage_quantity_checker
            .stub_beverage_as_available(&Beverage::Coffee(HotBeverageOption::Standard));
        stub_beverage_quantity_checker.stub_beverage_as_empty(Beverage::OrangeJuice);
        let mut machine = MachineBuilder::default()
            .set(Rc::new(DummyBeverageServer {}))
            .set(stub_beverage_quantity_checker)
            .set(Rc::new(DummyDisplay {}))
            .set(Rc::clone(&reports_printer_test_double))
            .set(Rc::new(DummyNotifier {}))
            .build();
        machine.dispense(BeverageRequest::new(
            &Beverage::Coffee(HotBeverageOption::Standard),
            &SugarAmount::Zero,
            ENOUGH_MONEY,
        ));
        machine.dispense(BeverageRequest::new(
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
        let stub_beverage_quantity_checker = Rc::new(BeverageQuantityCheckerTestDouble::new());
        stub_beverage_quantity_checker
            .stub_beverage_as_available(&Beverage::Coffee(HotBeverageOption::Standard));
        stub_beverage_quantity_checker.stub_beverage_as_empty(Beverage::OrangeJuice);
        stub_beverage_quantity_checker
            .stub_beverage_as_empty(Beverage::Tea(HotBeverageOption::ExtraHot));
        let notifier_test_double = Rc::new(NotifierTestDouble::new());
        let mut machine = MachineBuilder::default()
            .set(Rc::new(DummyBeverageServer {}))
            .set(stub_beverage_quantity_checker)
            .set(Rc::new(DummyDisplay {}))
            .set(Rc::new(DummyReportsPrinter {}))
            .set(Rc::clone(&notifier_test_double))
            .build();

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
        machine.dispense(coffee_beverage_request);
        machine.dispense(orange_juice_beverage_request);
        machine.dispense(tea_beverage_request);

        let notified_missing_beverages =
            notifier_test_double.spied_missing_beverages_notifications();
        assert_eq!(
            notified_missing_beverages,
            vec![
                Beverage::OrangeJuice,
                Beverage::Tea(HotBeverageOption::ExtraHot)
            ]
        )
    }
}
