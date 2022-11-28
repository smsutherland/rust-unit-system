use super::{CompositeUnit, UnitKind};
use crate::quantity::SingleQuantity;
use std::marker::PhantomData;
use std::ops::{Div, Mul};
use typenum::{Prod, Quot};

pub(super) trait ToSingle {
    type Single;
}

#[derive(PartialEq)]
pub struct SingleUnit<Kind: UnitKind> {
    _kind_marker: PhantomData<Kind>,
    pub(super) scale: f32,
    pub(super) abbreviation: &'static str,
    pub(super) name: &'static str,
}

impl<Kind: UnitKind> std::fmt::Debug for SingleUnit<Kind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.abbreviation)
    }
}

impl<Kind: UnitKind> SingleUnit<Kind> {
    pub fn abbreviation(&self) -> &'static str {
        self.abbreviation
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl<Kind1: UnitKind, Kind2: UnitKind> Mul<SingleUnit<Kind2>> for SingleUnit<Kind1>
where
    Kind1: Mul<Kind2>,
    Prod<Kind1, Kind2>: UnitKind,
{
    type Output = CompositeUnit<Prod<Kind1, Kind2>>;

    fn mul(self, rhs: SingleUnit<Kind2>) -> Self::Output {
        CompositeUnit::new(vec![(self.into(), 1), (rhs.into(), 1)])
    }
}

impl<Kind1: UnitKind, Kind2: UnitKind> Mul<CompositeUnit<Kind2>> for SingleUnit<Kind1>
where
    Kind2: Mul<Kind1>,
    Prod<Kind2, Kind1>: UnitKind,
{
    type Output = CompositeUnit<Prod<Kind2, Kind1>>;

    fn mul(self, rhs: CompositeUnit<Kind2>) -> Self::Output {
        rhs * self
    }
}

impl<Kind1: UnitKind, Kind2: UnitKind> Div<SingleUnit<Kind2>> for SingleUnit<Kind1>
where
    Kind1: Div<Kind2>,
    Quot<Kind1, Kind2>: UnitKind,
{
    type Output = CompositeUnit<Quot<Kind1, Kind2>>;

    fn div(self, rhs: SingleUnit<Kind2>) -> Self::Output {
        CompositeUnit::new(vec![(self.into(), 1), (rhs.into(), -1)])
    }
}

impl<Kind: UnitKind> Mul<f32> for SingleUnit<Kind> {
    type Output = SingleQuantity<Kind>;

    fn mul(self, rhs: f32) -> Self::Output {
        SingleQuantity::new(self.into(), rhs)
    }
}

impl<Kind: UnitKind> Mul<SingleUnit<Kind>> for f32 {
    type Output = SingleQuantity<Kind>;

    fn mul(self, rhs: SingleUnit<Kind>) -> Self::Output {
        SingleQuantity::new(rhs.into(), self)
    }
}

#[allow(non_upper_case_globals)]
pub const m: SingleUnit<super::Length> = SingleUnit {
    _kind_marker: PhantomData,
    scale: 1.,
    abbreviation: "m",
    name: "meter",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_m2() {
        let m2 = m * m;
        println!("{:?}", m2);
    }
    
    #[test]
    fn make_m3() {
        let m2 = m * m;
        let m3 = m2 * m;
        println!("{}", m3);


        let m2 = m * m;
        let m3 = m * m2;
        println!("{}", m3);
    }
}
