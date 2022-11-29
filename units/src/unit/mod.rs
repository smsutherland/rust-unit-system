mod composite;
pub use composite::CompositeUnit;
mod single;
pub use single::SingleUnit;
pub use single::{cd, kg, m, mol, s, A, K};
use std::ops::{Div, Mul};
use typenum::{Prod, Quot};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DynKind {
    length: i8,
    mass: i8,
    time: i8,
    current: i8,
    temperature: i8,
    amount: i8,
    luminosiy: i8,
}

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

pub trait UnitKind {
    fn to_dynkind() -> DynKind;
}

trait UnitFmt {
    fn abbrevation() -> &'static str;
    fn name() -> &'static str;
}

macro_rules! define_base_unit {
    ($name:ident, $kind:ident, $compunit:ident) => {
        pub struct $name;

        impl $name {
            fn to_comp() -> composite::$compunit {
                composite::$compunit::default()
            }
        }

        impl<Kind: UnitKind> Mul<Kind> for $name
        where
            composite::$compunit: Mul<Kind>,
        {
            type Output = Prod<composite::$compunit, Kind>;
            fn mul(self, rhs: Kind) -> Self::Output {
                Self::to_comp() * rhs
            }
        }

        impl<Kind: UnitKind> Div<Kind> for $name
        where
            composite::$compunit: Div<Kind>,
        {
            type Output = Quot<composite::$compunit, Kind>;
            fn div(self, rhs: Kind) -> Self::Output {
                Self::to_comp() / rhs
            }
        }

        impl UnitKind for $name {
            fn to_dynkind() -> DynKind {
                DynKind {
                    $kind: 1,
                    ..DynKind::default()
                }
            }
        }

        impl composite::IntoComp for $name {
            type Output = composite::$compunit;

            fn into(self) -> composite::$compunit {
                Default::default()
            }
        }
    };
}

define_base_unit!(Length, length, CompLength);
define_base_unit!(Mass, mass, CompMass);
define_base_unit!(Time, time, CompTime);
define_base_unit!(Current, current, CompCurrent);
define_base_unit!(Temperature, temperature, CompTemperature);
define_base_unit!(Amount, amount, CompAmount);
define_base_unit!(Luminosity, luminosiy, CompLuminosity);

#[cfg(test)]
mod tests {
    use super::*;
}
