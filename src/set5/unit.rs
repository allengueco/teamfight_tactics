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