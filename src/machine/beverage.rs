#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum Beverage {
    Coffee(HotBeverageOption),
    Tea(HotBeverageOption),
    HotChocolate(HotBeverageOption),
    OrangeJuice,
}



#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum HotBeverageOption {
    Standard,
    ExtraHot,
}
