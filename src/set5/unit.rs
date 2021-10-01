use crate::tft::unit::Unit;
use crate::set5::synergy::*;
pub fn garen() -> Unit<SynergySet5, 2> {
    Unit { name: "Garen", synergies: [Dawnbringer, Knight], cost: 5 }
}

pub fn kayle() -> Unit<SynergySet5, 2> {
    Unit { name: "Kayle", synergies: [Legionnaire, Redeemed], cost: 5 }
}

pub fn gwen() -> Unit<SynergySet5, 2> {
    Unit { name: "Gwen", synergies: [Inanimate, Mystic], cost: 5 }
}

pub fn akshan() -> Unit<SynergySet5, 2> {
    Unit { name: "Akshan", synergies: [Sentinel, Ranger], cost: 5 }
}

pub fn volibear() -> Unit<SynergySet5, 2> {
    Unit { name: "Volibear", synergies: [Revenant, Brawler], cost: 5 }
}