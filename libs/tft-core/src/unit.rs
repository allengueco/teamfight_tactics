use crate::synergy::Synergy;

#[derive(Debug)]
pub struct Unit<S: Synergy> {
    pub name: &'static str,
    pub synergies: Vec<S>,
    pub cost: u8,
}
