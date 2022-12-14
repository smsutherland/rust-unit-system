use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, LitStr, Token};

#[proc_macro]
pub fn create_unit(input: TokenStream) -> TokenStream {
    let unit = parse_macro_input!(input as Unit);
    todo!();
}

struct Unit {
    abbreviation: LitStr,
    name: LitStr,
}

impl Parse for Unit {
    fn parse(input: ParseStream) -> Result<Self> {
        let abbreviation = input.parse::<LitStr>()?;
        input.parse::<Token![,]>()?;
        let name = input.parse::<LitStr>()?;
        input.parse::<Token![,]>()?;
        let expr = input.parse::<Expr>()?;

        Ok(Self { abbreviation, name })
    }
}
