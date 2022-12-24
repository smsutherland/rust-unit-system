use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, BinOp, Error, Expr, Ident, LitStr, Result, Token, Type,
};

const METRIC_PREFIXES: [(f32, (&str, &str)); 21] = [
    (1e24, ("Y", "yotta")),
    (1e21, ("Z", "zetta")),
    (1e18, ("E", "exa")),
    (1e15, ("P", "peta")),
    (1e12, ("T", "tera")),
    (1e9, ("G", "giga")),
    (1e6, ("M", "mega")),
    (1e3, ("k", "kilo")),
    (1e2, ("h", "hecto")),
    (1e1, ("da", "deka")),
    (1e0, ("", "")),
    (1e-1, ("d", "deci")),
    (1e-2, ("c", "centi")),
    (1e-3, ("m", "milli")),
    (1e-6, ("μ", "micro")),
    (1e-9, ("n", "nano")),
    (1e-12, ("p", "pico")),
    (1e-15, ("f", "femto")),
    (1e-18, ("a", "atto")),
    (1e-21, ("z", "zepto")),
    (1e-24, ("y", "yocto")),
];

#[inline]
pub fn create_unit(input: TokenStream) -> TokenStream {
    let UnitDef {
        ident,
        abbreviation,
        name,
        tipe,
        initializer,
    } = parse_macro_input!(input as UnitDef);
    let scale = match create_scale(&initializer) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    quote! {
        #[allow(non_upper_case_globals)]
        pub const #ident: #tipe = #tipe {
            _kind_marker: ::std::marker::PhantomData,
            abbreviation: #abbreviation,
            name: #name,
            scale: #scale,
        };
    }
    .into()
}

#[inline]
pub fn create_unit_with_prefixes(input: TokenStream) -> TokenStream {
    let unit_def = parse_macro_input!(input as UnitDef);
    let scale = match create_scale(&unit_def.initializer) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    let units = METRIC_PREFIXES
        .into_iter()
        .map(|(scale_factor, prefix)| create_unit_prefix(&unit_def, prefix, &scale, scale_factor));
    quote! {
        #(#units)*
    }
    .into()
}

fn create_unit_prefix(
    unit_def: &UnitDef,
    prefix: (&'static str, &'static str),
    original_scale: &TokenStream2,
    scale_factor: f32,
) -> TokenStream2 {
    let ident = Ident::new(
        &format!("{}{}", prefix.0, unit_def.ident),
        unit_def.ident.span(),
    );
    let tipe = unit_def.tipe.clone();
    let abbreviation = LitStr::new(
        &format!("{}{}", prefix.0, unit_def.abbreviation.value()),
        unit_def.ident.span(),
    );
    let name = LitStr::new(
        &format!("{}{}", prefix.1, unit_def.name.value()),
        unit_def.ident.span(),
    );
    let scale = quote! {#scale_factor * (#original_scale)};

    quote! {
        #[allow(non_upper_case_globals)]
        pub const #ident: #tipe = #tipe {
            _kind_marker: ::std::marker::PhantomData,
            abbreviation: #abbreviation,
            name: #name,
            scale: #scale,
        };
    }
}

struct UnitDef {
    ident: Ident,
    abbreviation: LitStr,
    name: LitStr,
    tipe: Type,
    initializer: Expr,
}

impl Parse for UnitDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let tipe = input.parse::<Type>()?;
        input.parse::<Token![=]>()?;
        let initializer = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;
        let name = input.parse::<LitStr>()?;
        let abbreviation = match input.parse::<LitStr>() {
            Ok(a) => a,
            Err(_) => LitStr::new(&ident.to_string(), ident.span()),
        };
        Ok(Self {
            ident,
            abbreviation,
            name,
            tipe,
            initializer,
        })
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
