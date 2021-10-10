pub use lib_tft_core::Unit;
use crate::synergy::*;
pub fn garen() -> Unit<SynergySet5> {
    Unit { name: "Garen", synergies: vec![Dawnbringer, Knight], cost: 5 }
}

pub fn kayle() -> Unit<SynergySet5> {
    Unit { name: "Kayle", synergies: vec![Legionnaire, Redeemed], cost: 5 }
}

pub fn gwen() -> Unit<SynergySet5> {
    Unit { name: "Gwen", synergies: vec![Inanimate, Mystic], cost: 5 }
}

pub fn akshan() -> Unit<SynergySet5> {
    Unit { name: "Akshan", synergies: vec![Sentinel, Ranger], cost: 5 }
}

pub fn volibear() -> Unit<SynergySet5> {
    Unit { name: "Volibear", synergies: vec![Revenant, Brawler], cost: 5 }
}