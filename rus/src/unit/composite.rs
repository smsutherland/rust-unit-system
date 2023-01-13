use super::{single::ToSingle, DynUnit, SingleUnit, UnitKind};
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Div, Mul};
use typenum::{Prod, Quot};

/// Represents a product and/or quotient of many units.
///
/// For example, `m/s` is a composite unit.
/// Composite units can be created using arithmetic on other units.
/// `m/s` can be created as `u::m / u::s`.
#[derive(Debug)]
pub struct CompositeUnit<Kind: UnitKind> {
    pub(super) component_units: Vec<(DynUnit, i8)>,
    _kind_marker: PhantomData<Kind>,
}

impl<Kind: UnitKind> CompositeUnit<Kind> {
    pub(super) fn new(units: Vec<(DynUnit, i8)>) -> Self {
        Self {
            component_units: units,
            _kind_marker: PhantomData,
        }
    }

    pub(crate) fn scale_factor(&self) -> f32 {
        let mut res = 1.;
        for (unit, power) in &self.component_units {
            for _ in 0..*power {
                res *= unit.scale;
            }
        }
        res
    }
}

impl<Kind: UnitKind> PartialEq for CompositeUnit<Kind> {
    fn eq(&self, other: &Self) -> bool {
        self.component_units == other.component_units
    }
}

impl<Kind: UnitKind> Clone for CompositeUnit<Kind> {
    fn clone(&self) -> Self {
        Self {
            component_units: self.component_units.clone(),
            _kind_marker: PhantomData,
        }
    }
}

impl<Kind: UnitKind> ToSingle for CompositeUnit<Kind> {
    type Single = SingleUnit<Kind>;
}

impl<Kind1: UnitKind, Kind2: UnitKind> Mul<CompositeUnit<Kind2>> for CompositeUnit<Kind1>
where
    Kind1: Mul<Kind2>,
    Prod<Kind1, Kind2>: UnitKind,
{
    type Output = CompositeUnit<Prod<Kind1, Kind2>>;
    fn mul(mut self, rhs: CompositeUnit<Kind2>) -> Self::Output {
        let mut new_units = Vec::new();
        for (unit2, power2) in &rhs.component_units {
            let mut matched_unit = false;
            for (unit1, power1) in &mut self.component_units {
                if unit1 == unit2 {
                    *power1 += *power2;
                    matched_unit = false;
                    break;
                }
            }
            if !matched_unit {
                new_units.push((*unit2, *power2));
            }
        }
        self.component_units.append(&mut new_units);
        CompositeUnit {
            component_units: self.component_units,
            _kind_marker: PhantomData,
        }
    }
}

impl<Kind1: UnitKind, Kind2: UnitKind> Div<CompositeUnit<Kind2>> for CompositeUnit<Kind1>
where
    Kind1: Div<Kind2>,
    Quot<Kind1, Kind2>: UnitKind,
{
    type Output = CompositeUnit<Quot<Kind1, Kind2>>;
    fn div(mut self, rhs: CompositeUnit<Kind2>) -> Self::Output {
        let mut new_units = Vec::new();
        for (unit2, power2) in &rhs.component_units {
            let mut matched_unit = false;
            for (unit1, power1) in &mut self.component_units {
                if unit1 == unit2 {
                    *power1 -= *power2;
                    matched_unit = false;
                    break;
                }
            }
            if !matched_unit {
                new_units.push((*unit2, -*power2));
            }
        }
        self.component_units.append(&mut new_units);
        CompositeUnit {
            component_units: self.component_units,
            _kind_marker: PhantomData,
        }
    }
}

impl<Kind1: UnitKind, Kind2: UnitKind> Mul<SingleUnit<Kind2>> for CompositeUnit<Kind1>
where
    Kind1: Mul<Kind2>,
    Prod<Kind1, Kind2>: UnitKind,
{
    type Output = CompositeUnit<Prod<Kind1, Kind2>>;

    fn mul(mut self, rhs: SingleUnit<Kind2>) -> Self::Output {
        let rhs = rhs.into();
        for (i, (unit, power)) in self.component_units.iter_mut().enumerate() {
            if *unit == rhs {
                if *power == -1 {
                    self.component_units.swap_remove(i);
                } else {
                    *power += 1;
                }
                return CompositeUnit {
                    _kind_marker: PhantomData,
                    component_units: self.component_units,
                };
            }
        }
        self.component_units.push((rhs, 1));
        CompositeUnit {
            _kind_marker: PhantomData,
            component_units: self.component_units,
        }
    }
}

impl<Kind1: UnitKind, Kind2: UnitKind> Div<SingleUnit<Kind2>> for CompositeUnit<Kind1>
where
    Kind1: Div<Kind2>,
    Quot<Kind1, Kind2>: UnitKind,
{
    type Output = CompositeUnit<Quot<Kind1, Kind2>>;

    fn div(mut self, rhs: SingleUnit<Kind2>) -> Self::Output {
        let rhs = rhs.into();
        for (i, (unit, power)) in self.component_units.iter_mut().enumerate() {
            if *unit == rhs {
                if *power == 1 {
                    self.component_units.swap_remove(i);
                } else {
                    *power -= 1;
                }
                return CompositeUnit {
                    _kind_marker: PhantomData,
                    component_units: self.component_units,
                };
            }
        }
        self.component_units.push((rhs, -1));
        CompositeUnit {
            _kind_marker: PhantomData,
            component_units: self.component_units,
        }
    }
}

impl<Kind: UnitKind> From<SingleUnit<Kind>> for CompositeUnit<Kind> {
    fn from(other: SingleUnit<Kind>) -> Self {
        CompositeUnit {
            _kind_marker: PhantomData,
            component_units: vec![(other.into(), 1)],
        }
    }
}

impl<Kind: UnitKind> Display for CompositeUnit<Kind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, (unit, power)) in self.component_units.iter().enumerate() {
            match (i, power) {
                (_, 0) => {}
                (0, 1) => write!(f, "{}", unit.abbreviation)?,
                (0, _) => write!(f, "{}^{}", unit.abbreviation, power)?,
                (_, 1) => write!(f, " {}", unit.abbreviation)?,
                (_, _) => write!(f, " {}^{}", unit.abbreviation, power)?,
            }
        }
        Ok(())
    }
}

pub trait IntoComp: Sized {
    type Output;

    fn into(self) -> Self::Output;
}
