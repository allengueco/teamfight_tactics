use crate::tft::unit::{SynergyBreakpoint};

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
pub enum Synergy {
    // Origins
    Sentinel,
    Revenant,
    Redeemed,
    Nightbringer,
    Ironclad,
    Inanimate,
    Hellion,
    Forgotten,
    Draconic,
    Dawnbringer,
    Abomination,
    Victorious,

    // Classes
    Spellweaver,
    Skirmisher,
    Renewer,
    Ranger,
    Mystic,
    Legionnaire,
    Knight,
    Invoker,
    Cruel,
    Cavalier,
    Caretaker,
    Cannoneer,
    Brawler,
    Assassin,
}
use Synergy::*;
impl crate::tft::unit::Synergy for Synergy {
    fn interval(&self) -> SynergyBreakpoint {
        match self {
            Synergy::Sentinel => "3/6/9",
            Synergy::Revenant => "2/3/4/5/",
            Synergy::Redeemed => "3/6/9",
            Synergy::Nightbringer => "2/4/6/8",
            Synergy::Ironclad => "2/3/4/5",
            Synergy::Inanimate => "1",
            Synergy::Hellion => "2/4/6/8",
            Synergy::Forgotten => "2/4/6/8",
            Synergy::Draconic => "3/5",
            Synergy::Dawnbringer => "2/4/6/8",
            Synergy::Abomination => "3/4/5",
            Synergy::Victorious => "1",
            Synergy::Spellweaver => "2/4/6",
            Synergy::Skirmisher => "3/6/9",
            Synergy::Renewer => "2/4/6",
            Synergy::Ranger => "2/4/6",
            Synergy::Mystic => "2/3/4/5",
            Synergy::Legionnaire => "2/4/6/8",
            Synergy::Knight => "2/4/6",
            Synergy::Invoker => "1",
            Synergy::Cruel => "1",
            Synergy::Cavalier => "2/3/4/5",
            Synergy::Caretaker => "1",
            Synergy::Cannoneer => "2/4/6",
            Synergy::Brawler => "2/4/6",
            Synergy::Assassin => "2/4/6",
        }
        .into()
    }
}

struct Item {
    name: &'static str,
    description: &'static str,
}
impl crate::tft::item::Item for Item {
    fn name(&self) -> &str {
        self.name
    }
    fn description(&self) -> &str {
        self.description
    }
}

pub fn garen() -> crate::tft::unit::Unit<Synergy, 2> {
    let syns = [Dawnbringer, Knight];
    crate::tft::unit::Unit { name: "Garen", synergies: syns, cost: 5 }
}

pub fn kayle() -> crate::tft::unit::Unit<Synergy, 2> {
    let syns = [Legionnaire, Redeemed];
    crate::tft::unit::Unit { name: "Kayle", synergies: syns, cost: 5 }
}

#[cfg(test)]
mod tests {
    
}
