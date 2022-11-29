use super::{CompositeUnit, UnitKind};
use crate::quantity::SingleQuantity;
use std::marker::PhantomData;
use std::ops::{Div, Mul};
use typenum::{Prod, Quot};

pub(super) trait ToSingle {
    type Single;
}

#[derive(PartialEq, Clone, Copy)]
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
        let rhs = rhs.into();
        let dyn_self = self.into();
        if dyn_self == rhs {
            CompositeUnit::new(vec![(dyn_self, 2)])
        } else {
            CompositeUnit::new(vec![(dyn_self, 1), (rhs, 1)])
        }
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
        let rhs = rhs.into();
        let dyn_self = self.into();
        if dyn_self == rhs {
            CompositeUnit::new(Vec::new())
        } else {
            CompositeUnit::new(vec![(dyn_self, 1), (rhs, -1)])
        }
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

macro_rules! make_base_unit {
    ($name:ident, $longname:ident, $kind:ident) => {
        #[allow(non_upper_case_globals)]
        pub const $name: SingleUnit<super::$kind> = SingleUnit {
            _kind_marker: PhantomData,
            scale: 1.,
            abbreviation: stringify!($name),
            name: stringify!($longname),
        };
    };

    ($name:ident, $longname:literal, $kind:ident) => {
        #[allow(non_upper_case_globals)]
        pub const $name: SingleUnit<super::$kind> = SingleUnit {
            _kind_marker: PhantomData,
            scale: 1.,
            abbreviation: stringify!($name),
            name: $longname,
        };
    };
}

make_base_unit!(m, meter, Length);
make_base_unit!(kg, kilogram, Mass);
make_base_unit!(s, second, Time);
make_base_unit!(A, ampere, Current);
make_base_unit!(K, kelvin, Temperature);
make_base_unit!(mol, mole, Amount);
make_base_unit!(cd, candela, Luminosity);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_m2() {
        let m2 = m * m;
        println!("{}", m2);
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
