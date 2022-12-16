use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};
use typenum::{tarr, Diff, Integer, Sum, P1, Z0};
use units_macros::type_arith;

use super::composite::IntoComp;

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

pub trait UnitKind {
    fn to_dynkind() -> DynKind;
}

pub struct CompositeUnitKind<Length, Mass, Time, Current, Temperature, Amount, Luminosity> {
    #[allow(clippy::type_complexity)]
    _marker: PhantomData<tarr![Length, Mass, Time, Current, Temperature, Amount, Luminosity]>,
}

impl<L, M, T, C, Te, A, Lu> std::fmt::Debug for CompositeUnitKind<L, M, T, C, Te, A, Lu> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CompositeUnitKind")
    }
}

impl<L, M, T, C, Te, A, Lu> Default for CompositeUnitKind<L, M, T, C, Te, A, Lu> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
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

pub type LengthKind = CompositeUnitKind<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type MassKind = CompositeUnitKind<Z0, P1, Z0, Z0, Z0, Z0, Z0>;
pub type TimeKind = CompositeUnitKind<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
pub type CurrentKind = CompositeUnitKind<Z0, Z0, Z0, P1, Z0, Z0, Z0>;
pub type TemperatureKind = CompositeUnitKind<Z0, Z0, Z0, Z0, P1, Z0, Z0>;
pub type AmountKind = CompositeUnitKind<Z0, Z0, Z0, Z0, Z0, P1, Z0>;
pub type LuminosityKind = CompositeUnitKind<Z0, Z0, Z0, Z0, Z0, Z0, P1>;

pub type ForceKind = type_arith!(LengthKind * MassKind / TimeKind / TimeKind);
