use std::collections::HashMap;

use super::beverage::Beverage;

pub trait ReportsPrinter {
    fn print(&self, purchase_report: PurchasesReport);
}

#[derive(Debug, PartialEq, Eq)]
pub struct PurchasesReport {
    pub beverages_quantities: HashMap<Beverage, u32>,
    pub total_money_earned: u32,
}
