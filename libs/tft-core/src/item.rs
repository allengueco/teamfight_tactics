pub trait Item: Copy + Clone {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

pub trait MakeItem<I: Item> {
    fn make(item1: I, item2: I) -> I;
}
