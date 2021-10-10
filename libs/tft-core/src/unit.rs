use crate::synergy::Synergy;

#[derive(Debug)]
pub struct Unit<S: Synergy, const N: usize> {
    pub name: &'static str,
    pub synergies: [S; N],
    pub cost: u8,
}
