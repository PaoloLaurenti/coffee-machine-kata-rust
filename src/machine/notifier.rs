use super::beverage::Beverage;

pub trait Notifier {
    fn notify_missing_beverage(&self, beverage: &Beverage);
}
