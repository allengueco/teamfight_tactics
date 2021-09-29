

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
            Steps(Vec<u8>)
        }

        impl From<&str> for SynergyBreakpoint {
            fn from(str: &str) -> Self {
                let vec = str
                    .split('/')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<u8>>();

                match vec.len() {
                    1 => SynergyBreakpoint::Unique,
                    _ => SynergyBreakpoint::Steps(vec)
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
