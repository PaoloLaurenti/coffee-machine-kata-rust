pub mod drink_maker;
pub mod machine_system;

pub mod prelude {
    pub use crate::drink_maker::drink_maker_beverage_server::DrinkMakerBeverageServer;
    pub use crate::drink_maker::drink_maker_display::DrinkMakerDisplay;
    pub use crate::drink_maker::DrinkMaker;
    pub use crate::machine_system::beverages::beverage::*;
    pub use crate::machine_system::beverages::beverage_quantity_checker::BeverageQuantityChecker;
    pub use crate::machine_system::beverages::beverage_request::BeverageRequest;
    pub use crate::machine_system::beverages::beverage_server::BeverageServer;
    pub use crate::machine_system::beverages::sugar_amount::SugarAmount;
    pub use crate::machine_system::display::Display;
    pub use crate::machine_system::machine::Machine;
    pub use crate::machine_system::machine_builder::*;
    pub use crate::machine_system::notifier::Notifier;
    pub use crate::machine_system::reports_printer::PurchasesReport;
    pub use crate::machine_system::reports_printer::ReportsPrinter;
}
