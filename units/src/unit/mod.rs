mod composite;
pub use composite::CompositeUnit;
mod single;
pub use single::SingleUnit;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DynKind {
    length: i8,
    mass: i8,
    time: i8,
    current: i8,
    temperature: i8,
    amount: i8,
    luminosiy: i8,
}

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
            kind: Kind::to_dynkind(),
            scale: other.scale,
            abbreviation: other.abbreviation,
            name: other.name,
        }
    }
}

pub trait UnitKind {
    fn to_dynkind() -> DynKind;
}

trait UnitFmt {
    fn abbrevation() -> &'static str;
    fn name() -> &'static str;
}

struct Length;
impl UnitKind for Length {
    fn to_dynkind() -> DynKind {
        DynKind {
            length: 1,
            ..DynKind::default()
        }
    }
}
