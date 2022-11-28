use super::DynKind;
use super::{single::ToSingle, DynUnit, SingleUnit, UnitKind};
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};
use typenum::{Diff, Integer, Prod, Quot, Sum, P1, Z0};

#[derive(Debug, Clone)]
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
                new_units.push((unit2.clone(), -*power2));
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

#[derive(Debug)]
pub struct CompositeUnitKind<Length, Mass, Time, Current, Temperature, Amount, Luminosity> {
    _length_marker: PhantomData<Length>,
    _mass_marker: PhantomData<Mass>,
    _time_marker: PhantomData<Time>,
    _current_marker: PhantomData<Current>,
    _temperature_marker: PhantomData<Temperature>,
    _amount_marker: PhantomData<Amount>,
    _luminosity_marker: PhantomData<Luminosity>,
}

impl<L, M, T, C, Te, A, Lu> Default for CompositeUnitKind<L, M, T, C, Te, A, Lu> {
    fn default() -> Self {
        Self {
            _length_marker: PhantomData,
            _mass_marker: PhantomData,
            _time_marker: PhantomData,
            _current_marker: PhantomData,
            _temperature_marker: PhantomData,
            _amount_marker: PhantomData,
            _luminosity_marker: PhantomData,
        }
    }
}

impl<L: Integer, M: Integer, T: Integer, C: Integer, Te: Integer, A: Integer, Lu: Integer> UnitKind
    for CompositeUnitKind<L, M, T, C, Te, A, Lu>
{
    fn to_dynkind() -> DynKind {
        DynKind {
            length: L::to_i8(),
            mass: M::to_i8(),
            time: T::to_i8(),
            current: C::to_i8(),
            temperature: Te::to_i8(),
            amount: A::to_i8(),
            luminosiy: Lu::to_i8(),
        }
    }
}

impl<L1, M1, T1, C1, Te1, A1, Lu1, L2, M2, T2, C2, Te2, A2, Lu2>
    Mul<CompositeUnitKind<L2, M2, T2, C2, Te2, A2, Lu2>>
    for CompositeUnitKind<L1, M1, T1, C1, Te1, A1, Lu1>
where
    L1: Add<L2>,
    M1: Add<M2>,
    T1: Add<T2>,
    C1: Add<C2>,
    Te1: Add<Te2>,
    A1: Add<A2>,
    Lu1: Add<Lu2>,
{
    type Output = CompositeUnitKind<
        Sum<L1, L2>,
        Sum<M1, M2>,
        Sum<T1, T2>,
        Sum<C1, C2>,
        Sum<Te1, Te2>,
        Sum<A1, A2>,
        Sum<Lu1, Lu2>,
    >;

    fn mul(self, _: CompositeUnitKind<L2, M2, T2, C2, Te2, A2, Lu2>) -> Self::Output {
        Self::Output::default()
    }
}

impl<L1, M1, T1, C1, Te1, A1, Lu1, L2, M2, T2, C2, Te2, A2, Lu2, Kind2> Mul<Kind2>
    for CompositeUnitKind<L1, M1, T1, C1, Te1, A1, Lu1>
where
    L1: Add<L2>,
    M1: Add<M2>,
    T1: Add<T2>,
    C1: Add<C2>,
    Te1: Add<Te2>,
    A1: Add<A2>,
    Lu1: Add<Lu2>,
    Kind2: IntoComp<Output = CompositeUnitKind<L2, M2, T2, C2, Te2, A2, Lu2>>,
{
    type Output = CompositeUnitKind<
        Sum<L1, L2>,
        Sum<M1, M2>,
        Sum<T1, T2>,
        Sum<C1, C2>,
        Sum<Te1, Te2>,
        Sum<A1, A2>,
        Sum<Lu1, Lu2>,
    >;

    fn mul(self, _: Kind2) -> Self::Output {
        Self::Output::default()
    }
}

impl<L1, M1, T1, C1, Te1, A1, Lu1, L2, M2, T2, C2, Te2, A2, Lu2>
    Div<CompositeUnitKind<L2, M2, T2, C2, Te2, A2, Lu2>>
    for CompositeUnitKind<L1, M1, T1, C1, Te1, A1, Lu1>
where
    L1: Sub<L2>,
    M1: Sub<M2>,
    T1: Sub<T2>,
    C1: Sub<C2>,
    Te1: Sub<Te2>,
    A1: Sub<A2>,
    Lu1: Sub<Lu2>,
{
    type Output = CompositeUnitKind<
        Diff<L1, L2>,
        Diff<M1, M2>,
        Diff<T1, T2>,
        Diff<C1, C2>,
        Diff<Te1, Te2>,
        Diff<A1, A2>,
        Diff<Lu1, Lu2>,
    >;

    fn div(self, _: CompositeUnitKind<L2, M2, T2, C2, Te2, A2, Lu2>) -> Self::Output {
        Self::Output::default()
    }
}

impl<L1, M1, T1, C1, Te1, A1, Lu1, L2, M2, T2, C2, Te2, A2, Lu2, Kind2> Div<Kind2>
    for CompositeUnitKind<L1, M1, T1, C1, Te1, A1, Lu1>
where
    L1: Sub<L2>,
    M1: Sub<M2>,
    T1: Sub<T2>,
    C1: Sub<C2>,
    Te1: Sub<Te2>,
    A1: Sub<A2>,
    Lu1: Sub<Lu2>,
    Kind2: IntoComp<Output = CompositeUnitKind<L2, M2, T2, C2, Te2, A2, Lu2>>,
{
    type Output = CompositeUnitKind<
        Diff<L1, L2>,
        Diff<M1, M2>,
        Diff<T1, T2>,
        Diff<C1, C2>,
        Diff<Te1, Te2>,
        Diff<A1, A2>,
        Diff<Lu1, Lu2>,
    >;

    fn div(self, _: Kind2) -> Self::Output {
        Self::Output::default()
    }
}

pub type CompLength = CompositeUnitKind<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type CompMass = CompositeUnitKind<Z0, P1, Z0, Z0, Z0, Z0, Z0>;
pub type CompTime = CompositeUnitKind<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
pub type CompCurrent = CompositeUnitKind<Z0, Z0, Z0, P1, Z0, Z0, Z0>;
pub type CompTemperature = CompositeUnitKind<Z0, Z0, Z0, Z0, P1, Z0, Z0>;
pub type CompAmount = CompositeUnitKind<Z0, Z0, Z0, Z0, Z0, P1, Z0>;
pub type CompLuminosity = CompositeUnitKind<Z0, Z0, Z0, Z0, Z0, Z0, P1>;

pub trait IntoComp: Sized {
    type Output;

    fn into(self) -> Self::Output;
}
