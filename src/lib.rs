

mod tft {
    pub(crate) mod unit {
        #[derive(Debug)]
        pub struct Unit<S: Synergy, const N: usize> {
            pub name: &'static str,
            pub synergies: [S; N],
            pub cost: u8,
        }
        pub trait Synergy {
            fn interval(&self) -> SynergyBreakpoint;
        }
        pub enum SynergyBreakpoint {
            Unique,
            One { start: u8, n: u8 },
            Two { start: u8, n: u8 },
            Three { start: u8, n: u8 },
        }

        impl From<&str> for SynergyBreakpoint {
            fn from(str: &str) -> Self {
                let vec = str
                    .split('/')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<u8>>();
                let n = vec.len() as u8;
                let start = vec[0];

                match vec.as_slice() {
                    [_] => SynergyBreakpoint::Unique,
                    [s @ ..] => match s[1] - s[0] {
                        1 => SynergyBreakpoint::One { start, n },
                        2 => SynergyBreakpoint::Two { start, n },
                        3 => SynergyBreakpoint::Three { start, n },
                        x => panic!("invalid breakpoint interval: {}", x),
                    },
                }
            }
        }
        pub(crate) struct Cost(pub u8);
    }
    pub(crate) mod item {
        pub trait Item {
            fn name(&self) -> &str;
            fn description(&self) -> &str;
        }
    }

    pub(crate) mod board {
        pub trait Board {}
    }
}
mod set5;


// macro_rules! synergy {
//             (
//                 $( $syn:ident => $($breakpoint:expr);+ ),+
//             ) => {
//                 enum Synergy {
//                     $($syn)+,
//                 }
//                 impl crate::tft::unit::Synergy for Synergy {
//                     fn interval(&self) -> Self {
//                         match self {
//                             $($syn => stringify!($breakpoint),)+
//                         }
//                     }
//                 }
//             };
//         }
//
// synergy!(
//     victorious => 1,
//     redeemed => 3;6;9
// );
//
// units! {
//     garen => 5 victorious,
//     kayle => 5 legionnaire redeemed,
//     miss_fortune => cannoneer forgotten,
// }


#[cfg(test)]
mod tests {
    use super::*;

    mod unit {
        use super::*;

        #[test]
        fn synergy_from_str() {}
    }
}
