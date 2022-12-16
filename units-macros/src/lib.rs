use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, BinOp, Error, Expr, LitStr, Token, Type};

#[proc_macro]
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
        let kind = create_unit_kind(&self.expr)?;
        let scale = create_scale(&self.expr)?;
        Ok(quote! {
            const #abbreviation: SingleUnit<#kind> = SingleUnit<#kind> {
                _kind_marker: PhantomData,
                scale: #scale,
                abbreviation: #abbreviation,
                name: #name,
            };
            macro_rules! #abbreviation {
                () => {
                    SingleUnit<#kind>
                };
            }

        }
        .into())
    }
}

fn create_unit_kind(expr: &Expr) -> Result<Type> {
    match expr {
        Expr::Binary(expr) => {
            let left = create_unit_kind(expr.left.as_ref())?;
            let right = create_unit_kind(expr.right.as_ref())?;
            match expr.op {
                BinOp::Mul(_) => Ok(Type::Verbatim(quote! {
                    <#left as ::std::ops::Mul<#right>>::Output
                })),
                BinOp::Div(_) => Ok(Type::Verbatim(quote! {
                    <#left as ::std::ops::Div<#right>>::Output
                })),
                other => Err(Error::new_spanned(
                    other,
                    "Only supported operaors are `*` and `/`",
                )),
            }
        }
        Expr::Lit(expr) => Ok(Type::Verbatim(quote! {})),
        _ => Err(Error::new_spanned(
            expr,
            "expected binary operator expression",
        )),
    }
}

fn create_scale(expr: &Expr) -> Result<Expr> {
    todo!();
}

#[proc_macro]
pub fn type_arith(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    match handle_expr(&expr) {
        Ok(result) => result.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn handle_expr(expr: &Expr) -> Result<TokenStream2> {
    match expr {
        Expr::Binary(expr) => {
            let left = handle_expr(expr.left.as_ref())?;
            let right = handle_expr(expr.right.as_ref())?;

            match expr.op {
                BinOp::Mul(_) => Ok(quote! {
                    <#left as ::core::ops::Mul<#right>>::Output
                }),
                BinOp::Div(_) => Ok(quote! {
                    <#left as ::core::ops::Div<#right>>::Output
                }),
                other => Err(Error::new_spanned(
                    other,
                    "only handling `*` and `/` operators",
                )),
            }
        }
        Expr::Path(expr) => Ok(quote! {#expr}),
        _ => Err(Error::new_spanned(
            expr,
            "expected binary operator expression",
        )),
    }
}
