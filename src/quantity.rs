use crate::unit::{CompositeUnit, SingleUnit};
use std::fmt::Display;
use std::ops::Mul;

#[derive(Debug)]
pub struct SingleQuantity {
    unit: CompositeUnit,
    scalar: f32,
}

impl SingleQuantity {
    pub fn new(unit: CompositeUnit, scalar: f32) -> Self {
        Self { unit, scalar }
    }
}

impl Mul for SingleQuantity {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.unit * rhs.unit, self.scalar * rhs.scalar)
    }
}

impl Mul<SingleUnit> for SingleQuantity {
    type Output = Self;

    fn mul(self, rhs: SingleUnit) -> Self::Output {
        Self {
            unit: self.unit * rhs,
            ..self
        }
    }
}

impl Display for SingleQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.scalar, self.unit)
    }
}
