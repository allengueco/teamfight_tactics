pub trait Synergy {
    fn interval(&self) -> SynergyBreakpoint;
}

pub enum SynergyBreakpoint {
    Unique,
    Steps(Vec<u8>)
}

impl From<&str> for SynergyBreakpoint {
    fn from(str: &str) -> Self {
        let vec = str
            .split('/')
            .map(|s| s.parse().expect("Cannot parse breakpoint"))
            .collect::<Vec<u8>>();

        match vec.len() {
            1 => SynergyBreakpoint::Unique,
            _ => SynergyBreakpoint::Steps(vec)
        }
    }
}