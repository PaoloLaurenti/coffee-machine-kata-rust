#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Beverage {
    Coffee(HotBeverageOption),
    Tea(HotBeverageOption),
    HotChocolate(HotBeverageOption),
    OrangeJuice,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum HotBeverageOption {
    Standard,
    ExtraHot,
}
