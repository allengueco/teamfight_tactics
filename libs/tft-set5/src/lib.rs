use std::path::Path;

pub use item::*;
use lib_tft_parse::{Creator, Set};
pub use r#trait::*;
pub use unit::*;

pub(crate) mod item;
pub(crate) mod r#trait;
pub(crate) mod unit;

pub struct Set5 {
    units: Vec<Set5Unit>,
    items: Vec<Set5Item>,
    traits: Vec<Set5Trait>,
}
impl Creator<Set5Unit> for Set5 {}
impl Creator<Set5Item> for Set5 {}
impl Creator<Set5Trait> for Set5 {}

impl Set<Set5Unit, Set5Item, Set5Trait> for Set5 {}

impl Set5 {
    pub fn new<P: AsRef<Path>>(
        champions_file_path: P,
        items_file_path: P,
        traits_file_path: P,
    ) -> Self {
        Self {
            units: Self::units(champions_file_path),
            items: Self::items(items_file_path),
            traits: Self::traits(traits_file_path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_set5() {
        let s5: Set5 = Set5::new(
            Path::new("src/resources/champions.json"),
            Path::new("src/resources/items.json"),
            Path::new("src/resources/traits.json"),
        );
        assert!(!s5.units.is_empty());
        assert!(!s5.items.is_empty());
        assert!(!s5.traits.is_empty());
    }
}
