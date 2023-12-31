use std::collections::HashMap;

use super::beverages::beverage::Beverage;

pub trait ReportsPrinter {
    fn print(&self, purchase_report: PurchasesReport);
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PurchasesReport {
    pub beverages_quantities: HashMap<Beverage, u32>,
    pub total_money_earned: u32,
}

impl PurchasesReport {
    pub fn new(beverages_quantities: &HashMap<Beverage, u32>, total_money_earned: u32) -> Self {
        Self {
            beverages_quantities: beverages_quantities.clone(),
            total_money_earned,
        }
    }
}
