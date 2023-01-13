use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, BinOp, Error, Expr, Result};

#[inline]
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
