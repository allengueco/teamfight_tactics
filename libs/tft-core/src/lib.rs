pub(crate) mod unit;
pub(crate) mod item;
pub(crate) mod synergy;

pub use unit::*;
pub use item::*;
pub use synergy::*;


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
