use crate::unit::kind::UnitKind;
use crate::unit::{CompositeUnit, SingleUnit};
use std::fmt::Display;
use std::ops::Mul;
use typenum::Prod;

#[derive(Debug, Clone)]
pub struct SingleQuantity<Kind: UnitKind> {
    unit: CompositeUnit<Kind>,
    scalar: f32,
}

impl<Kind: UnitKind> SingleQuantity<Kind> {
    pub fn new(unit: CompositeUnit<Kind>, scalar: f32) -> Self {
        Self { unit, scalar }
    }

    pub fn to(&self, unit: impl Into<CompositeUnit<Kind>>) -> Self {
        let unit = unit.into();
        let source_scale = self.unit.scale_factor();
        let target_scale = unit.scale_factor();
        Self {
            unit,
            scalar: self.scalar * target_scale / source_scale,
        }
    }
}

impl<Kind1: UnitKind, Kind2: UnitKind> Mul<SingleQuantity<Kind2>> for SingleQuantity<Kind1>
where
    Kind1: Mul<Kind2>,
    Prod<Kind1, Kind2>: UnitKind,
{
    type Output = SingleQuantity<Prod<Kind1, Kind2>>;

    fn mul(self, rhs: SingleQuantity<Kind2>) -> Self::Output {
        Self::Output::new(self.unit * rhs.unit, self.scalar * rhs.scalar)
    }
}

impl<Kind1: UnitKind, Kind2: UnitKind> Mul<SingleUnit<Kind2>> for SingleQuantity<Kind1>
where
    Kind1: Mul<Kind2>,
    Prod<Kind1, Kind2>: UnitKind,
{
    type Output = SingleQuantity<Prod<Kind1, Kind2>>;

    fn mul(self, rhs: SingleUnit<Kind2>) -> Self::Output {
        Self::Output {
            unit: self.unit * rhs,
            scalar: self.scalar,
        }
    }
}

impl<Kind: UnitKind> Display for SingleQuantity<Kind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.scalar, self.unit)
    }
}
