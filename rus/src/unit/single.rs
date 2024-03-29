use super::{CompositeUnit, UnitKind};
use crate::quantity::SingleQuantity;
use std::marker::PhantomData;
use std::ops::{Div, Mul};
use typenum::{Prod, Quot};

pub(super) trait ToSingle {
    type Single;
}

/// A individual unit, such as a second.
///
/// This does not have to represent a base unit. For example, a Newton is also a single unit.
#[derive(PartialEq, Clone, Copy)]
pub struct SingleUnit<Kind: UnitKind> {
    _kind_marker: PhantomData<Kind>,
    /// The scale from the SI base unit equivalent.
    pub scale: f32,
    /// An abbreviation for the unit.
    pub abbreviation: &'static str,
    /// The full name of the unit.
    pub name: &'static str,
}

impl<Kind: UnitKind> std::fmt::Debug for SingleUnit<Kind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SingleUnit")
            .field("scale", &self.scale)
            .field("abbreviation", &self.abbreviation)
            .field("name", &self.name)
            .finish()
    }
}

impl<Kind: UnitKind> std::fmt::Display for SingleUnit<Kind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.abbreviation)
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

pub mod kinds {
    use super::SingleUnit;
    use crate::unit::kind::*;

    pub type LengthUnit = SingleUnit<LengthKind>;
    pub type MassUnit = SingleUnit<MassKind>;
    pub type TimeUnit = SingleUnit<TimeKind>;
    pub type CurrentUnit = SingleUnit<CurrentKind>;
    pub type TemperatureUnit = SingleUnit<TemperatureKind>;
    pub type AmountUnit = SingleUnit<AmountKind>;
    pub type LuminosityUnit = SingleUnit<LuminosityKind>;

    pub type ForceUnit = SingleUnit<ForceKind>;
}

/// Module containing the definitions of all units.
///
/// Units are defined in this module to lessen clutter in the root module. When using this library,
/// it is recommended to `use units::unit_defs as u` and then use `u::cm` in order to access all the units.
pub mod unit_defs {
    use super::kinds::*;
    use rus_macros::{create_unit, create_unit_with_prefixes};

    create_unit_with_prefixes!(
        /// A meter. The SI unit of length.
        m: LengthUnit = 1.,
        "meter"
    );
    create_unit_with_prefixes!(
        /// A gram. This is not the SI unit of mass. Instead, the kilogram is the SI unit of mass.
        /// A gram is one thousandth of a kilogram.
        ///
        /// Due to how prefixed units are generated, currently the specific documentation cannot go
        /// on the kilogram.
        g: MassUnit = 1e-3,
        "gram"
    );
    create_unit_with_prefixes!(
        /// A second. The SI unit of time.
        s: TimeUnit = 1.,
        "second"
    );
    create_unit_with_prefixes!(
        /// An Ampere. The SI unit of current.
        A: CurrentUnit = 1.,
        "ampere"
    );
    create_unit_with_prefixes!(
        /// A Kelvin. The SI unit of temperature.
        K: TemperatureUnit = 1.,
        "kelvin"
    );
    create_unit_with_prefixes!(
        /// A mole. The SI unit of amount.
        mole: AmountUnit = 1.,
        "mole"
    );
    create_unit_with_prefixes!(
        /// A candela. The SI unit of luminous intensity.
        cd: LuminosityUnit = 1.,
        "candela"
    );

    create_unit!(
        /// A Newton. The derived unit of force.
        N: ForceUnit = kg * m / (s * s),
        "Newton"
    );
}

#[cfg(test)]
mod test {
    use super::unit_defs::*;
    #[test]
    fn multiple_meter() {
        let m2 = m * m;
        println!("{}", m2);
    }

    #[test]
    fn use_km() {
        println!("{}", km);
    }
}
