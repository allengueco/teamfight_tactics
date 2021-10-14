use std::path::Path;

pub use item::*;
use lib_tft_parse::{Creator, Set};
pub use r#trait::*;
pub use unit::*;

pub(crate) mod unit;
pub(crate) mod item;
pub(crate) mod r#trait;

pub struct ChampionCreator;

impl Creator for ChampionCreator {
    type Type = Set5Unit;
}

pub struct ItemCreator;

impl Creator for ItemCreator {
    type Type = Set5Item;
}

pub struct TraitCreator;

impl Creator for TraitCreator {
    type Type = Set5Trait;
}

pub struct Set5 {
    units: Vec<Set5Unit>,
    items: Vec<Set5Item>,
    traits: Vec<Set5Trait>,
}

impl Set<Set5Unit, Set5Item, Set5Trait, ChampionCreator, ItemCreator, TraitCreator> for Set5 {}

impl Set5 {
    pub fn new<P: AsRef<Path>>(
        champions_file_path: P,
        items_file_path: P,
        traits_file_path: P,
    ) -> Self {
        Self {
            units: Set5::units(champions_file_path),
            items: Set5::items(items_file_path),
            traits: Set5::traits(traits_file_path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod unit {
        use super::*;
    }

    mod item {
        use super::*;
    }

    mod r#trait {
        use super::*;
    }
}