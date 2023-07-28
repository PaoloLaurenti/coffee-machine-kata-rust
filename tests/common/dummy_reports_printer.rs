use coffee_machine_kata_rust::machine::reports_printer::{PurchasesReport, ReportsPrinter};

pub(crate) struct DummyReportsPrinter {}
impl ReportsPrinter for DummyReportsPrinter {
    fn print(&self, _purchase_report: PurchasesReport) {}
}
