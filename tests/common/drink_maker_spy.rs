use coffee_machine_kata_rust::drink_maker::DrinkMaker;
use std::cell::RefCell;

pub(crate) struct DrinkMakerSpy {
    received_commands: RefCell<Vec<String>>,
}

impl DrinkMakerSpy {
    pub(crate) fn new() -> DrinkMakerSpy {
        DrinkMakerSpy {
            received_commands: RefCell::new(vec![]),
        }
    }

    pub(crate) fn get_received_commands(&self) -> Vec<String> {
        self.received_commands.clone().take()
    }
}

impl DrinkMaker for DrinkMakerSpy {
    fn execute(&self, command: String) {
        self.received_commands.borrow_mut().push(command);
    }
}
