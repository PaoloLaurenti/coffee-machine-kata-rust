use super::beverage::Beverage;

pub trait Notifier {
    fn notify_missing_drink(&self, drink: &Beverage);
}
