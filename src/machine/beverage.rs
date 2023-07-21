#[derive(Eq, Hash, PartialEq)]
pub enum Beverage {
    Coffee(HotBeverageOption),
    Tea(HotBeverageOption),
    HotChocolate(HotBeverageOption),
    OrangeJuice,
}

#[derive(Eq, Hash, PartialEq)]
pub enum HotBeverageOption {
    Standard,
    ExtraHot,
}
