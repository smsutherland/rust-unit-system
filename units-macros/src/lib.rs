use proc_macro::TokenStream;
mod create_unit;
mod create_unit_with_prefixes;
mod type_arith;

#[proc_macro]
pub fn create_unit(input: TokenStream) -> TokenStream {
    create_unit::create_unit(input)
}

#[proc_macro]
pub fn type_arith(input: TokenStream) -> TokenStream {
    type_arith::type_arith(input)
}

#[proc_macro]
pub fn create_unit_with_prefixes(input: TokenStream) -> TokenStream {
    create_unit_with_prefixes::create_unit_with_prefixes(input)
}
