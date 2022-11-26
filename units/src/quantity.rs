use crate::unit::{CompositeUnit, SingleUnit};
use std::fmt::Display;
use std::ops::{Add, Mul};
use typenum::{Integer, Sum};

#[derive(Debug, Clone)]
pub struct SingleQuantity<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
where
    Length: Integer,
    Mass: Integer,
    Time: Integer,
    Current: Integer,
    Temperature: Integer,
    Amount: Integer,
    Luminosity: Integer,
{
    unit: CompositeUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>,
    scalar: f32,
}

impl<L: Integer, M: Integer, T: Integer, C: Integer, Te: Integer, A: Integer, Lu: Integer>
    SingleQuantity<L, M, T, C, Te, A, Lu>
{
    pub fn new(unit: CompositeUnit<L, M, T, C, Te, A, Lu>, scalar: f32) -> Self {
        Self { unit, scalar }
    }

    pub fn to(&self, unit: impl Into<CompositeUnit<L, M, T, C, Te, A, Lu>>) -> Self {
        let unit = unit.into();
        let source_scale = self.unit.scale_factor();
        let target_scale = unit.scale_factor();
        Self {
            unit,
            scalar: self.scalar * target_scale / source_scale,
        }
    }
}

impl<
        L1: Integer + Add<L2>,
        M1: Integer + Add<M2>,
        T1: Integer + Add<T2>,
        C1: Integer + Add<C2>,
        Te1: Integer + Add<Te2>,
        A1: Integer + Add<A2>,
        Lu1: Integer + Add<Lu2>,
        L2: Integer,
        M2: Integer,
        T2: Integer,
        C2: Integer,
        Te2: Integer,
        A2: Integer,
        Lu2: Integer,
    > Mul<SingleQuantity<L1, M1, T1, C1, Te1, A1, Lu1>>
    for SingleQuantity<L2, M2, T2, C2, Te2, A2, Lu2>
where
    Sum<L1, L2>: Integer,
    Sum<M1, M2>: Integer,
    Sum<T1, T2>: Integer,
    Sum<C1, C2>: Integer,
    Sum<Te1, Te2>: Integer,
    Sum<A1, A2>: Integer,
    Sum<Lu1, Lu2>: Integer,
{
    type Output = SingleQuantity<
        Sum<L1, L2>,
        Sum<M1, M2>,
        Sum<T1, T2>,
        Sum<C1, C2>,
        Sum<Te1, Te2>,
        Sum<A1, A2>,
        Sum<Lu1, Lu2>,
    >;

    fn mul(self, rhs: SingleQuantity<L1, M1, T1, C1, Te1, A1, Lu1>) -> Self::Output {
        Self::Output::new(self.unit * rhs.unit, self.scalar * rhs.scalar)
    }
}

impl<
        L1: Integer + Add<L2>,
        M1: Integer + Add<M2>,
        T1: Integer + Add<T2>,
        C1: Integer + Add<C2>,
        Te1: Integer + Add<Te2>,
        A1: Integer + Add<A2>,
        Lu1: Integer + Add<Lu2>,
        L2: Integer,
        M2: Integer,
        T2: Integer,
        C2: Integer,
        Te2: Integer,
        A2: Integer,
        Lu2: Integer,
    > Mul<SingleUnit<L1, M1, T1, C1, Te1, A1, Lu1>> for SingleQuantity<L2, M2, T2, C2, Te2, A2, Lu2>
where
    Sum<L1, L2>: Integer,
    Sum<M1, M2>: Integer,
    Sum<T1, T2>: Integer,
    Sum<C1, C2>: Integer,
    Sum<Te1, Te2>: Integer,
    Sum<A1, A2>: Integer,
    Sum<Lu1, Lu2>: Integer,
{
    type Output = SingleQuantity<
        Sum<L1, L2>,
        Sum<M1, M2>,
        Sum<T1, T2>,
        Sum<C1, C2>,
        Sum<Te1, Te2>,
        Sum<A1, A2>,
        Sum<Lu1, Lu2>,
    >;

    fn mul(self, rhs: SingleUnit<L1, M1, T1, C1, Te1, A1, Lu1>) -> Self::Output {
        Self::Output {
            unit: self.unit * rhs,
            scalar: self.scalar,
        }
    }
}

impl<L: Integer, M: Integer, T: Integer, C: Integer, Te: Integer, A: Integer, Lu: Integer> Display
    for SingleQuantity<L, M, T, C, Te, A, Lu>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.scalar, self.unit)
    }
}
