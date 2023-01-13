pub mod composite;
pub use composite::CompositeUnit;
pub mod kind;
use kind::*;
pub mod single;
pub use single::SingleUnit;
// pub mod equivalency;

#[derive(Debug, Clone, Copy, PartialEq)]
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

trait UnitFmt {
    fn abbrevation() -> &'static str;
    fn name() -> &'static str;
}
