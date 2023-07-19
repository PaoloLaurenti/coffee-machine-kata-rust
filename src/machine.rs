use crate::drink_maker::DrinkMaker;

pub enum BeverageType {
    Coffe,
    Tea,
    HotChocolate,
}

pub enum SugarAmount {
    Zero,
    One,
    Two,
}

pub struct BeverageRequest {
    beverage_type: BeverageType,
    sugar_amount: SugarAmount,
    _money_amount: u32,
}

impl BeverageRequest {
    pub fn new(
        beverage_type: BeverageType,
        sugar_amount: SugarAmount,
        _money_amount: u32,
    ) -> BeverageRequest {
        BeverageRequest {
            beverage_type,
            sugar_amount,
            _money_amount: 0,
        }
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
        self.drink_maker.execute(drink_maker_cmd)
    }
}

fn build_drink_maker_command(beverage_request: BeverageRequest) -> String {
    let beverage_cmd_part = match beverage_request.beverage_type {
        BeverageType::Coffe => "C",
        BeverageType::Tea => "T",
        BeverageType::HotChocolate => "H",
    };

    let (sugar_amount_cmd_part, stick_cmd_part) = match beverage_request.sugar_amount {
        SugarAmount::Zero => ("", ""),
        SugarAmount::One => ("1", "0"),
        SugarAmount::Two => ("2", "0"),
    };

    format!("{beverage_cmd_part}:{sugar_amount_cmd_part}:{stick_cmd_part}")
}

#[cfg(test)]
mod machine_tests {
    use super::*;
    use std::cell::RefCell;
    use test_case::test_case;

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
            self.received_commands.clone().take()
        }
    }

    impl DrinkMaker for DrinkMakerSpy {
        fn execute(&self, command: String) {
            self.received_commands.borrow_mut().push(command);
        }
    }

    #[test_case(BeverageType::Coffe, "C::" ; "cofee")]
    #[test_case(BeverageType::Tea, "T::" ; "tea")]
    #[test_case(BeverageType::HotChocolate, "H::" ; "hot chocolate")]
    fn machine_dispenses_beverage_with_no_sugar_no_stick(
        beverage_type: BeverageType,
        expected_drink_maker_cmd: &str,
    ) {
        let drink_maker_spy = DrinkMakerSpy::new();
        let machine = Machine::new(&drink_maker_spy);

        let beverage_request = BeverageRequest::new(beverage_type, SugarAmount::Zero, 100000);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(
            drink_maker_cmds,
            vec![String::from(expected_drink_maker_cmd)]
        )
    }

    #[test_case(SugarAmount::One, "1" ; "one sugar")]
    #[test_case(SugarAmount::Two, "2" ; "two sugars")]
    fn machine_dispenses_beverage_with_one_sugar(
        sugar_amount: SugarAmount,
        expected_sugar_amount_cmd_part: &str,
    ) {
        let drink_maker_spy = DrinkMakerSpy::new();
        let machine = Machine::new(&drink_maker_spy);

        let beverage_request = BeverageRequest::new(BeverageType::Coffe, sugar_amount, 100000);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(1, drink_maker_cmds.len());
        let sugar_amount_cmd_part = drink_maker_cmds[0].split(':').nth(1).unwrap();
        assert_eq!(sugar_amount_cmd_part, expected_sugar_amount_cmd_part)
    }

    #[test_case(SugarAmount::One, "0" ; "stick with one sugar")]
    #[test_case(SugarAmount::Two, "0" ; "stick with two sugars")]
    fn machine_dispenses_beverage_with_stick_when_some_sugar_is_requested(
        sugar_amount: SugarAmount,
        expected_stick_cmd_part: &str,
    ) {
        let drink_maker_spy = DrinkMakerSpy::new();
        let machine = Machine::new(&drink_maker_spy);

        let beverage_request = BeverageRequest::new(BeverageType::Coffe, sugar_amount, 100000);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(1, drink_maker_cmds.len());
        let stick_cmd_part = drink_maker_cmds[0].split(':').nth(2).unwrap();
        assert_eq!(stick_cmd_part, expected_stick_cmd_part)
    }

    #[test_case(BeverageType::Coffe, 60, "C::" ; "coffee costs 0.6€")]
    #[test_case(BeverageType::Tea, 40, "T::" ; "tea costs 0.4€")]
    #[test_case(BeverageType::HotChocolate, 50, "H::" ; "hot chocolate costs 0.5€")]
    fn machine_dispense_coffee_only_when_given_money_is_enough(
        beverage_type: BeverageType,
        money_amount: u32,
        expected_drink_maker_cmd: &str,
    ) {
        let drink_maker_spy = DrinkMakerSpy::new();
        let machine = Machine::new(&drink_maker_spy);

        let beverage_request = BeverageRequest::new(beverage_type, SugarAmount::Zero, money_amount);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(drink_maker_cmds, vec![String::from(expected_drink_maker_cmd)])
    }
}
