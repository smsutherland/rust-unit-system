use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, BinOp, Error, Expr, LitStr, Result, Token,
};

#[inline]
pub fn create_unit(input: TokenStream) -> TokenStream {
    let unit = parse_macro_input!(input as Unit);
    match unit.create_unit() {
        Ok(result) => result,
        Err(err) => err.to_compile_error().into(),
    }
}

struct Unit {
    abbreviation: LitStr,
    name: LitStr,
    expr: Expr,
}

impl Parse for Unit {
    fn parse(input: ParseStream) -> Result<Self> {
        let abbreviation = input.parse::<LitStr>()?;
        input.parse::<Token![,]>()?;
        let name = input.parse::<LitStr>()?;
        input.parse::<Token![,]>()?;
        let expr = input.parse::<Expr>()?;

        Ok(Self {
            abbreviation,
            name,
            expr,
        })
    }
}

impl Unit {
    fn create_unit(self) -> Result<TokenStream> {
        let abbreviation = self.abbreviation;
        let name = self.name;
        let scale = create_scale(&self.expr)?;
        Ok(quote! {
            SingleUnit {
                _kind_marker: PhantomData,
                scale: #scale,
                abbreviation: #abbreviation,
                name: #name,
            }
        }
        .into())
    }
}

fn create_scale(expr: &Expr) -> Result<TokenStream2> {
    match expr {
        Expr::Binary(expr) => {
            let left = create_scale(expr.left.as_ref())?;
            let right = create_scale(expr.right.as_ref())?;
            match expr.op {
                BinOp::Mul(_) => Ok(quote! {
                    (#left) * (#right)
                }),
                BinOp::Div(_) => Ok(quote! {
                    (#left) / (#right)
                }),
                other => Err(Error::new_spanned(
                    other,
                    "Only supported operaors are `*` and `/`",
                )),
            }
        }
        Expr::Lit(expr) => Ok(quote! {#expr}),
        Expr::Path(expr) => Ok(quote! {#expr.scale}),
        _ => Err(Error::new_spanned(
            expr,
            "expected binary operator expression",
        )),
    }
}
