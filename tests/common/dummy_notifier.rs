use coffee_machine_kata_rust::machine_system::{beverages::beverage::Beverage, notifier::Notifier};
pub(crate) struct DummyNotifier {}

impl Notifier for DummyNotifier {
    fn notify_missing_beverage(&self, _drink: &Beverage) {}
}
