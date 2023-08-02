use super::{
    beverages::{
        beverage_quantity_checker::BeverageQuantityChecker, beverage_server::BeverageServer,
        dispenser::Dispenser,
    },
    cashier::Cashier,
    display::Display,
    machine::Machine,
    notifier::Notifier,
    reports_printer::ReportsPrinter,
};

#[derive(Default)]
pub struct MachineBuilder {}

impl MachineBuilder {
    pub fn with_beverage_server(
        self,
        beverage_server: &impl BeverageServer,
    ) -> RequiresBeverageQuantityChecker {
        RequiresBeverageQuantityChecker::new(beverage_server)
    }
}

pub struct RequiresBeverageQuantityChecker<'a> {
    beverage_server: &'a dyn BeverageServer,
}

impl<'a> RequiresBeverageQuantityChecker<'a> {
    fn new(beverage_server: &'a impl BeverageServer) -> Self {
        Self { beverage_server }
    }

    pub fn with_beverage_quantity_checker(
        self,
        beverage_quantity_checker: &'a impl BeverageQuantityChecker,
    ) -> RequiresDisplay {
        RequiresDisplay::new(self, beverage_quantity_checker)
    }
}

pub struct RequiresDisplay<'a> {
    beverage_server: &'a dyn BeverageServer,
    beverage_quantity_checker: &'a dyn BeverageQuantityChecker,
}

impl<'a> RequiresDisplay<'a> {
    fn new(
        requires_beverage_quantity_checker: RequiresBeverageQuantityChecker<'a>,
        beverage_quantity_checker: &'a impl BeverageQuantityChecker,
    ) -> Self {
        Self {
            beverage_server: requires_beverage_quantity_checker.beverage_server,
            beverage_quantity_checker,
        }
    }

    pub fn with_display(self, display: &'a impl Display) -> RequiresReportsPrinter {
        RequiresReportsPrinter::new(self, display)
    }
}

pub struct RequiresReportsPrinter<'a> {
    beverage_server: &'a dyn BeverageServer,
    beverage_quantity_checker: &'a dyn BeverageQuantityChecker,
    display: &'a dyn Display,
}

impl<'a> RequiresReportsPrinter<'a> {
    fn new(requires_display: RequiresDisplay<'a>, display: &'a impl Display) -> Self {
        Self {
            beverage_server: requires_display.beverage_server,
            beverage_quantity_checker: requires_display.beverage_quantity_checker,
            display,
        }
    }

    pub fn with_reports_printer(self, report_printer: &'a impl ReportsPrinter) -> RequiresNotifier {
        RequiresNotifier::new(self, report_printer)
    }
}

pub struct RequiresNotifier<'a> {
    beverage_server: &'a dyn BeverageServer,
    beverage_quantity_checker: &'a dyn BeverageQuantityChecker,
    display: &'a dyn Display,
    reports_printer: &'a dyn ReportsPrinter,
}

impl<'a> RequiresNotifier<'a> {
    fn new(
        requires_reports_printer: RequiresReportsPrinter<'a>,
        reports_printer: &'a impl ReportsPrinter,
    ) -> Self {
        Self {
            beverage_server: requires_reports_printer.beverage_server,
            beverage_quantity_checker: requires_reports_printer.beverage_quantity_checker,
            display: requires_reports_printer.display,
            reports_printer,
        }
    }

    pub fn with_notifier(self, notifier: &'a impl Notifier) -> MachineBuilderReadyForBuilding {
        MachineBuilderReadyForBuilding::new(self, notifier)
    }
}

pub struct MachineBuilderReadyForBuilding<'a> {
    beverage_server: &'a dyn BeverageServer,
    beverage_quantity_checker: &'a dyn BeverageQuantityChecker,
    display: &'a dyn Display,
    reports_printer: &'a dyn ReportsPrinter,
    notifier: &'a dyn Notifier,
}

impl<'a> MachineBuilderReadyForBuilding<'a> {
    fn new(requires_notifier: RequiresNotifier<'a>, notifier: &'a impl Notifier) -> Self {
        Self {
            beverage_server: requires_notifier.beverage_server,
            beverage_quantity_checker: requires_notifier.beverage_quantity_checker,
            display: requires_notifier.display,
            reports_printer: requires_notifier.reports_printer,
            notifier,
        }
    }

    pub fn build(self) -> Machine<'a> {
        Machine {
            dispenser: Dispenser::new(self.beverage_server, self.beverage_quantity_checker),
            cashier: Cashier::new(),
            display: self.display,
            reports_printer: self.reports_printer,
            notifier: self.notifier,
        }
    }
}

#[cfg(test)]
mod machine_builder_tests {
    use crate::machine_system::machine::machine_tests::{
        DummyBeverageServer, DummyDisplay, DummyNotifier, DummyReportsPrinter,
        InfiniteBeverageQuantityCheckerFake,
    };

    use super::MachineBuilder;

    #[test]
    fn build_a_machine() {
        MachineBuilder::default()
            .with_beverage_server(&DummyBeverageServer {})
            .with_beverage_quantity_checker(&InfiniteBeverageQuantityCheckerFake {})
            .with_display(&DummyDisplay {})
            .with_reports_printer(&DummyReportsPrinter {})
            .with_notifier(&DummyNotifier {})
            .build();
    }
}
