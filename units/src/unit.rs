use crate::quantity::SingleQuantity;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Div, Mul};
use typenum::{Prod, Quot};

#[derive(Debug, Clone)]
pub struct CompositeUnit<Kind: UnitKind> {
    component_units: Vec<(DynUnit, i8)>,
    _kind_marker: PhantomData<Kind>,
}

impl<Kind: UnitKind> CompositeUnit<Kind> {
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

pub trait ToSingle {
    type Single;
}

impl<Kind: UnitKind> ToSingle for CompositeUnit<Kind> {
    type Single = SingleUnit<Kind>;
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
                new_units.push((unit2.clone(), *power2));
            }
        }
        self.component_units.append(&mut new_units);
        CompositeUnit {
            component_units: self.component_units,
            _kind_marker: PhantomData,
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

#[derive(PartialEq)]
pub struct SingleUnit<Kind: UnitKind> {
    _kind_marker: PhantomData<Kind>,
    scale: f32,
    abbreviation: &'static str,
    name: &'static str,
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
        CompositeUnit {
            _kind_marker: PhantomData,
            component_units: vec![(self.into(), 1), (rhs.into(), 1)],
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
        CompositeUnit {
            _kind_marker: PhantomData,
            component_units: vec![(self.into(), 1), (rhs.into(), -1)],
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

#[derive(Debug, Clone, PartialEq)]
pub struct DynKind;

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
            kind: Kind::to_kind(),
            scale: other.scale,
            abbreviation: other.abbreviation,
            name: other.name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaseUnit<Kind: UnitKind> {
    scale_from_si: f32,
    _marker: PhantomData<Kind>,
}

pub trait UnitKind {
    fn to_kind() -> DynKind;
}

trait UnitFmt {
    fn abbrevation() -> &'static str;
    fn name() -> &'static str;
}
