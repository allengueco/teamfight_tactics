use proc_macro::TokenStream;
use syn::{DeriveInput, Data};
use quote::{quote};
trait Synergy {
    fn breakpoint(&self) -> SynergyBreakpoint;
}

enum SynergyBreakpoint {
    Unique,
    Steps(Vec<u8>)
}

pub fn derive(input: DeriveInput) -> TokenStream {
    let synergies = match input.data {
        Data::Enum(ref e) => {},
        _ => unimplemented!("Only enum supported for defining synergies")
    };

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse2;

    #[test]
    fn simple_synergy() {
        let input = parse2::<DeriveInput>(quote! {
            #[derive(Synergy)]
            enum TestSynergy {
                Invoker,
                Ranger,
            }
        }).unwrap();

        let actual = derive(input);

        assert_eq!(actual.to_string(),
        quote! {
            enum SynergyDecl {
                Dawnbringer,
                Ranger,
                Cruel,
            }

            impl Synergy for SynergyDecl {
                fn breakpoint(&self) -> SynergyBreakpoint {
                    match self {
                        Dawnbringer => SynergyBreakpoint::Steps(vec![2,4,6,8]),
                        Ranger => SynergyBreakpoint::Steps(vec![2,4,6]),
                        Cruel => SynergyBreakpoint::Unique
                    }
                }
            }
        }.to_string())

    }
}