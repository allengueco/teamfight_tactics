use lib_tft_core::{Synergy, SynergyBreakpoint};

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum SynergySet5 {
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
pub use SynergySet5::*;

impl Synergy for SynergySet5 {
    fn interval(&self) -> SynergyBreakpoint {
        match self {
            SynergySet5::Sentinel => "3/6/9",
            SynergySet5::Revenant => "2/3/4/5/",
            SynergySet5::Redeemed => "3/6/9",
            SynergySet5::Nightbringer => "2/4/6/8",
            SynergySet5::Ironclad => "2/3/4/5",
            SynergySet5::Inanimate => "1",
            SynergySet5::Hellion => "2/4/6/8",
            SynergySet5::Forgotten => "2/4/6/8",
            SynergySet5::Draconic => "3/5",
            SynergySet5::Dawnbringer => "2/4/6/8",
            SynergySet5::Abomination => "3/4/5",
            SynergySet5::Victorious => "1",
            SynergySet5::Spellweaver => "2/4/6",
            SynergySet5::Skirmisher => "3/6/9",
            SynergySet5::Renewer => "2/4/6",
            SynergySet5::Ranger => "2/4/6",
            SynergySet5::Mystic => "2/3/4/5",
            SynergySet5::Legionnaire => "2/4/6/8",
            SynergySet5::Knight => "2/4/6",
            SynergySet5::Invoker => "1",
            SynergySet5::Cruel => "1",
            SynergySet5::Cavalier => "2/3/4/5",
            SynergySet5::Caretaker => "1",
            SynergySet5::Cannoneer => "2/4/6",
            SynergySet5::Brawler => "2/4/6",
            SynergySet5::Assassin => "2/4/6",
        }
        .into()
    }
}