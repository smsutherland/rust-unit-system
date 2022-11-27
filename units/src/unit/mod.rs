use std::marker::PhantomData;
mod composite;
pub use composite::CompositeUnit;
mod single;
pub use single::SingleUnit;

#[derive(Debug, Clone, PartialEq)]
pub struct DynKind;

#[derive(Debug, Clone, PartialEq)]
struct DynUnit {
    kind: DynKind,
    scale: f32,
    abbreviation: &'static str,
    name: &'static str,
}

impl<Kind: UnitKind> From<SingleUnit<Kind>> for DynUnit {
    fn from(other: SingleUnit<Kind>) -> Self {
        Self {
            kind: Kind::to_kind(),
            scale: other.scale,
            abbreviation: other.abbreviation,
            name: other.name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaseUnit<Kind: UnitKind> {
    scale_from_si: f32,
    _marker: PhantomData<Kind>,
}

pub trait UnitKind {
    fn to_kind() -> DynKind;
}

trait UnitFmt {
    fn abbrevation() -> &'static str;
    fn name() -> &'static str;
}
