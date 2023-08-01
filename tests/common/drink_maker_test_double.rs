use coffee_machine_kata_rust::drink_maker::DrinkMaker;
use std::cell::RefCell;
pub(crate) struct DrinkMakerTestDouble {
    received_commands: RefCell<Vec<String>>,
}

impl DrinkMakerTestDouble {
    pub(crate) fn new() -> DrinkMakerTestDouble {
        DrinkMakerTestDouble {
            received_commands: RefCell::new(vec![]),
        }
    }

    pub(crate) fn spied_received_commands(&self) -> Vec<String> {
        self.received_commands.clone().take()
    }
}

impl DrinkMaker for DrinkMakerTestDouble {
    fn execute(&self, command: String) {
        self.received_commands.borrow_mut().push(command);
    }
}
