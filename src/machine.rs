use crate::drink_maker::DrinkMaker;

pub enum BeverageType {
    Coffe,
    Tea,
}

pub enum SugarQuantity {
    Zero,
}

pub struct BeverageRequest {
    beverage_type: BeverageType,
}

impl BeverageRequest {
    pub fn new(beverage_type: BeverageType, _sugar_quantity: SugarQuantity) -> BeverageRequest {
        BeverageRequest { beverage_type }
    }
}

pub struct Machine<'a> {
    drink_maker: &'a dyn DrinkMaker,
}

impl Machine<'_> {
    pub fn new(drink_maker: &dyn DrinkMaker) -> Machine {
        Machine { drink_maker }
    }

    pub fn dispense(&self, beverage_request: BeverageRequest) {
        let drink_maker_cmd = build_drink_maker_command(beverage_request);
        self.drink_maker.pour(drink_maker_cmd)
    }
}

fn build_drink_maker_command(beverage_request: BeverageRequest) -> String {
    match beverage_request.beverage_type {
        BeverageType::Coffe => String::from("C::"),
        BeverageType::Tea => String::from("T::"),
    }
}

#[cfg(test)]
mod machine_tests {
    use super::*;
    use std::cell::RefCell;

    struct DrinkMakerSpy {
        received_commands: RefCell<Vec<String>>,
    }

    impl DrinkMakerSpy {
        fn new() -> DrinkMakerSpy {
            DrinkMakerSpy {
                received_commands: RefCell::new(vec![]),
            }
        }

        pub fn get_received_commands(&self) -> Vec<String> {
            self.received_commands.take()
        }
    }

    impl DrinkMaker for DrinkMakerSpy {
        fn pour(&self, command: String) {
            self.received_commands.borrow_mut().push(command);
        }
    }

    #[test]
    fn machine_dispense_coffee_with_no_sugar() {
        let drink_maker_spy = DrinkMakerSpy::new();
        let machine = Machine::new(&drink_maker_spy);

        let beverage_request = BeverageRequest::new(BeverageType::Coffe, SugarQuantity::Zero);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(drink_maker_cmds, vec![String::from("C::")])
    }

    #[test]
    fn machine_dispense_tea_with_no_sugar() {
        let drink_maker_spy = DrinkMakerSpy::new();
        let machine = Machine::new(&drink_maker_spy);

        let beverage_request = BeverageRequest::new(BeverageType::Tea, SugarQuantity::Zero);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(drink_maker_cmds, vec![String::from("T::")])
    }
}
