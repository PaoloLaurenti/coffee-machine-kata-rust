use crate::drink_maker::DrinkMaker;

pub enum BeverageType {
    Coffee,
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
    money_amount: u32,
}

impl BeverageRequest {
    pub fn new(
        beverage_type: BeverageType,
        sugar_amount: SugarAmount,
        money_amount: u32,
    ) -> BeverageRequest {
        BeverageRequest {
            beverage_type,
            sugar_amount,
            money_amount,
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
        let beverage_type = &beverage_request.beverage_type;
        let money_amount = beverage_request.money_amount;
        let missing_money = how_much_money_is_missing(beverage_type, money_amount);

        if missing_money > 0 {
            let formatted_missing_money = missing_money as f32 / 100.0;
            self.drink_maker
                .execute(format!("M:{formatted_missing_money}€"));
            return;
        }

        let drink_maker_cmd =
            build_drink_maker_command(beverage_type, beverage_request.sugar_amount);
        self.drink_maker.execute(drink_maker_cmd)
    }
}

fn how_much_money_is_missing(beverage_type: &BeverageType, money_amount: u32) -> i32 {
    match beverage_type {
        BeverageType::Coffee => 60 - money_amount as i32,
        BeverageType::Tea => 40 - money_amount as i32,
        BeverageType::HotChocolate => 50 - money_amount as i32,
    }
}

fn build_drink_maker_command(beverage_type: &BeverageType, sugar_amount: SugarAmount) -> String {
    let beverage_cmd_part = match beverage_type {
        BeverageType::Coffee => "C",
        BeverageType::Tea => "T",
        BeverageType::HotChocolate => "H",
    };

    let (sugar_amount_cmd_part, stick_cmd_part) = match sugar_amount {
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

    #[test_case(BeverageType::Coffee, "C::" ; "cofee")]
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

        let beverage_request = BeverageRequest::new(BeverageType::Coffee, sugar_amount, 100000);
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

        let beverage_request = BeverageRequest::new(BeverageType::Coffee, sugar_amount, 100000);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(1, drink_maker_cmds.len());
        let stick_cmd_part = drink_maker_cmds[0].split(':').nth(2).unwrap();
        assert_eq!(stick_cmd_part, expected_stick_cmd_part)
    }

    #[test_case(BeverageType::Coffee, 60, "C::" ; "coffee costs 0.6€")]
    #[test_case(BeverageType::Tea, 40, "T::" ; "tea costs 0.4€")]
    #[test_case(BeverageType::HotChocolate, 50, "H::" ; "hot chocolate costs 0.5€")]
    fn machine_dispenses_beverages_only_when_given_money_is_enough(
        beverage_type: BeverageType,
        money_amount: u32,
        expected_drink_maker_cmd: &str,
    ) {
        let drink_maker_spy = DrinkMakerSpy::new();
        let machine = Machine::new(&drink_maker_spy);

        let beverage_request = BeverageRequest::new(beverage_type, SugarAmount::Zero, money_amount);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(
            drink_maker_cmds,
            vec![String::from(expected_drink_maker_cmd)]
        )
    }

    #[test_case(BeverageType::Coffee, 59, "C::"; "coffee costs 0.6€")]
    #[test_case(BeverageType::Tea, 39, "T::" ; "tea costs 0.4€")]
    #[test_case(BeverageType::HotChocolate, 49, "H::" ; "hot chocolate costs 0.5€")]
    fn machine_does_not_dispense_beverages_when_given_money_is_not_enough(
        beverage_type: BeverageType,
        money_amount: u32,
        dispense_drink_maker_cmd: &str,
    ) {
        let drink_maker_spy = DrinkMakerSpy::new();
        let machine = Machine::new(&drink_maker_spy);

        let beverage_request = BeverageRequest::new(beverage_type, SugarAmount::Zero, money_amount);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(
            false,
            drink_maker_cmds.contains(&dispense_drink_maker_cmd.to_string())
        )
    }

    #[test_case(BeverageType::Coffee, 59, "M:0.01€"; "coffee costs 0.6€, missing 0.01€")]
    #[test_case(BeverageType::Coffee, 1, "M:0.59€"; "coffee costs 0.6€, missing 0.59€")]
    #[test_case(BeverageType::Tea, 39, "M:0.01€"; "tea costs 0.4€, missing 0.01€")]
    #[test_case(BeverageType::Tea, 1, "M:0.39€"; "tea costs 0.4€, missing 0.39€")]
    #[test_case(BeverageType::HotChocolate, 49, "M:0.01€"; "tea costs 0.5€, missing 0.01€")]
    #[test_case(BeverageType::HotChocolate, 1, "M:0.49€"; "tea costs 0.5€, missing 0.49€")]
    fn machine_shows_missing_amount_when_asked_for_a_beverage_with_not_enough_money(
        beverage_type: BeverageType,
        money_amount: u32,
        expected_drink_maker_cmd: &str,
    ) {
        let drink_maker_spy = DrinkMakerSpy::new();
        let machine = Machine::new(&drink_maker_spy);

        let beverage_request = BeverageRequest::new(beverage_type, SugarAmount::Zero, money_amount);
        machine.dispense(beverage_request);

        let drink_maker_cmds = drink_maker_spy.get_received_commands();
        assert_eq!(
            drink_maker_cmds,
            vec![String::from(expected_drink_maker_cmd)]
        )
    }
}
