use super::beverages::beverage::Beverage;

pub trait Notifier {
    fn notify_missing_beverage(&self, beverage: &Beverage);
}
