use crate::quantity::SingleQuantity;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul};
use typenum::{Integer, Prod, Sum, P1, Z0};

#[derive(Debug)]
pub struct CompositeUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
where
    Length: Integer,
    Mass: Integer,
    Time: Integer,
    Current: Integer,
    Temperature: Integer,
    Amount: Integer,
    Luminosity: Integer,
{
    component_units: Vec<(DynUnit, i8)>,
    _length_marker: PhantomData<Length>,
    _mass_marker: PhantomData<Mass>,
    _time_marker: PhantomData<Time>,
    _current_marker: PhantomData<Current>,
    _temperature_marker: PhantomData<Temperature>,
    _amount_marker: PhantomData<Amount>,
    _luminosity_marker: PhantomData<Luminosity>,
}

pub trait ToSingle {
    type Single;
}

impl<L: Integer, M: Integer, T: Integer, C: Integer, Te: Integer, A: Integer, Lu: Integer> ToSingle
    for CompositeUnit<L, M, T, C, Te, A, Lu>
{
    type Single = SingleUnit<L, M, T, C, Te, A, Lu>;
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
    > Mul<CompositeUnit<L1, M1, T1, C1, Te1, A1, Lu1>>
    for CompositeUnit<L2, M2, T2, C2, Te2, A2, Lu2>
where
    Sum<L1, L2>: Integer,
    Sum<M1, M2>: Integer,
    Sum<T1, T2>: Integer,
    Sum<C1, C2>: Integer,
    Sum<Te1, Te2>: Integer,
    Sum<A1, A2>: Integer,
    Sum<Lu1, Lu2>: Integer,
{
    type Output = CompositeUnit<
        Sum<L1, L2>,
        Sum<M1, M2>,
        Sum<T1, T2>,
        Sum<C1, C2>,
        Sum<Te1, Te2>,
        Sum<A1, A2>,
        Sum<Lu1, Lu2>,
    >;

    fn mul(mut self, mut rhs: CompositeUnit<L1, M1, T1, C1, Te1, A1, Lu1>) -> Self::Output {
        // TODO: reduce units
        self.component_units.append(&mut rhs.component_units);
        CompositeUnit {
            component_units: self.component_units,
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

impl<
        L1: Integer + Add<L2>,
        M1: Integer + Add<M2>,
        T1: Integer + Add<T2>,
        C1: Integer + Add<C2>,
        Te1: Integer + Add<Te2>,
        Am1: Integer + Add<A2>,
        Lu1: Integer + Add<Lu2>,
        L2: Integer,
        M2: Integer,
        T2: Integer,
        C2: Integer,
        Te2: Integer,
        A2: Integer,
        Lu2: Integer,
    > Mul<SingleUnit<L1, M1, T1, C1, Te1, Am1, Lu1>> for CompositeUnit<L2, M2, T2, C2, Te2, A2, Lu2>
where
    Sum<L1, L2>: Integer,
    Sum<M1, M2>: Integer,
    Sum<T1, T2>: Integer,
    Sum<C1, C2>: Integer,
    Sum<Te1, Te2>: Integer,
    Sum<Am1, A2>: Integer,
    Sum<Lu1, Lu2>: Integer,
{
    type Output = CompositeUnit<
        Sum<L1, L2>,
        Sum<M1, M2>,
        Sum<T1, T2>,
        Sum<C1, C2>,
        Sum<Te1, Te2>,
        Sum<Am1, A2>,
        Sum<Lu1, Lu2>,
    >;

    fn mul(mut self, rhs: SingleUnit<L1, M1, T1, C1, Te1, Am1, Lu1>) -> Self::Output {
        let rhs = rhs.into();
        for (i, (unit, power)) in self.component_units.iter_mut().enumerate() {
            if *unit == rhs {
                if *power == -1 {
                    self.component_units.swap_remove(i);
                } else {
                    *power += 1;
                }
                return CompositeUnit {
                    component_units: self.component_units,
                    _length_marker: PhantomData,
                    _mass_marker: PhantomData,
                    _time_marker: PhantomData,
                    _current_marker: PhantomData,
                    _temperature_marker: PhantomData,
                    _amount_marker: PhantomData,
                    _luminosity_marker: PhantomData,
                };
            }
        }
        self.component_units.push((rhs, 1));
        CompositeUnit {
            component_units: self.component_units,
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

impl<L: Integer, M: Integer, T: Integer, C: Integer, Te: Integer, A: Integer, Lu: Integer>
    From<SingleUnit<L, M, T, C, Te, A, Lu>> for CompositeUnit<L, M, T, C, Te, A, Lu>
{
    fn from(other: SingleUnit<L, M, T, C, Te, A, Lu>) -> Self {
        CompositeUnit {
            component_units: vec![(other.into(), 1)],
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

impl<L: Integer, M: Integer, T: Integer, C: Integer, Te: Integer, A: Integer, Lu: Integer> Display
    for CompositeUnit<L, M, T, C, Te, A, Lu>
{
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

pub trait Pow<Power: Integer> {
    type Output;
}

#[derive(PartialEq)]
pub struct SingleUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
where
    Length: Integer,
    Mass: Integer,
    Current: Integer,
    Temperature: Integer,
    Amount: Integer,
    Luminosity: Integer,
{
    system: UnitSystem,
    scale: f32,
    abbreviation: &'static str,
    name: &'static str,
    _length_marker: PhantomData<Length>,
    _mass_marker: PhantomData<Mass>,
    _time_marker: PhantomData<Time>,
    _current_marker: PhantomData<Current>,
    _temperature_marker: PhantomData<Temperature>,
    _amount_marker: PhantomData<Amount>,
    _luminosity_marker: PhantomData<Luminosity>,
}

impl<L: Integer, M: Integer, T: Integer, C: Integer, Te: Integer, Am: Integer, Lu: Integer>
    std::fmt::Debug for SingleUnit<L, M, T, C, Te, Am, Lu>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.abbreviation)
    }
}

impl<
        L: Integer + Mul<Power>,
        M: Integer + Mul<Power>,
        T: Integer + Mul<Power>,
        C: Integer + Mul<Power>,
        Te: Integer + Mul<Power>,
        A: Integer + Mul<Power>,
        Lu: Integer + Mul<Power>,
        Power: Integer,
    > Pow<Power> for SingleUnit<L, M, T, C, Te, A, Lu>
where
    Prod<L, Power>: Integer,
    Prod<M, Power>: Integer,
    Prod<T, Power>: Integer,
    Prod<C, Power>: Integer,
    Prod<Te, Power>: Integer,
    Prod<A, Power>: Integer,
    Prod<Lu, Power>: Integer,
{
    type Output = SingleUnit<
        Prod<L, Power>,
        Prod<M, Power>,
        Prod<T, Power>,
        Prod<C, Power>,
        Prod<Te, Power>,
        Prod<A, Power>,
        Prod<Lu, Power>,
    >;
}

impl<L, M, T, C, Te, A, Lu> SingleUnit<L, M, T, C, Te, A, Lu>
where
    L: Integer,
    M: Integer,
    C: Integer,
    Te: Integer,
    A: Integer,
    Lu: Integer,
{
    pub fn abbreviation(&self) -> &'static str {
        self.abbreviation
    }

    pub fn name(&self) -> &'static str {
        self.name
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
    > Mul<SingleUnit<L1, M1, T1, C1, Te1, A1, Lu1>> for SingleUnit<L2, M2, T2, C2, Te2, A2, Lu2>
where
    Sum<L1, L2>: Integer,
    Sum<M1, M2>: Integer,
    Sum<T1, T2>: Integer,
    Sum<C1, C2>: Integer,
    Sum<Te1, Te2>: Integer,
    Sum<A1, A2>: Integer,
    Sum<Lu1, Lu2>: Integer,
{
    type Output = CompositeUnit<
        Sum<L1, L2>,
        Sum<M1, M2>,
        Sum<T1, T2>,
        Sum<C1, C2>,
        Sum<Te1, Te2>,
        Sum<A1, A2>,
        Sum<Lu1, Lu2>,
    >;

    fn mul(self, rhs: SingleUnit<L1, M1, T1, C1, Te1, A1, Lu1>) -> Self::Output {
        CompositeUnit {
            component_units: vec![(self.into(), 1), (rhs.into(), 1)],
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

impl<
        L1: Integer,
        M1: Integer,
        T1: Integer,
        C1: Integer,
        Te1: Integer,
        A1: Integer,
        Lu1: Integer,
        L2: Integer + Add<L1>,
        M2: Integer + Add<M1>,
        T2: Integer + Add<T1>,
        C2: Integer + Add<C1>,
        Te2: Integer + Add<Te1>,
        A2: Integer + Add<A1>,
        Lu2: Integer + Add<Lu1>,
    > Mul<CompositeUnit<L1, M1, T1, C1, Te1, A1, Lu1>> for SingleUnit<L2, M2, T2, C2, Te2, A2, Lu2>
where
    Sum<L2, L1>: Integer,
    Sum<M2, M1>: Integer,
    Sum<T2, T1>: Integer,
    Sum<C2, C1>: Integer,
    Sum<Te2, Te1>: Integer,
    Sum<A2, A1>: Integer,
    Sum<Lu2, Lu1>: Integer,
{
    type Output = CompositeUnit<
        Sum<L2, L1>,
        Sum<M2, M1>,
        Sum<T2, T1>,
        Sum<C2, C1>,
        Sum<Te2, Te1>,
        Sum<A2, A1>,
        Sum<Lu2, Lu1>,
    >;

    fn mul(self, rhs: CompositeUnit<L1, M1, T1, C1, Te1, A1, Lu1>) -> Self::Output {
        rhs * self
    }
}

impl<L: Integer, M: Integer, T: Integer, C: Integer, Te: Integer, A: Integer, Lu: Integer> Div
    for SingleUnit<L, M, T, C, Te, A, Lu>
{
    type Output = CompositeUnit<L, M, T, C, Te, A, Lu>;

    fn div(self, rhs: Self) -> Self::Output {
        CompositeUnit {
            component_units: vec![(self.into(), 1), (rhs.into(), -1)],
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

impl<L: Integer, M: Integer, T: Integer, C: Integer, Te: Integer, A: Integer, Lu: Integer> Mul<f32>
    for SingleUnit<L, M, T, C, Te, A, Lu>
{
    type Output = SingleQuantity<L, M, T, C, Te, A, Lu>;

    fn mul(self, rhs: f32) -> Self::Output {
        SingleQuantity::new(self.into(), rhs)
    }
}

impl<L: Integer, M: Integer, T: Integer, C: Integer, Te: Integer, A: Integer, Lu: Integer>
    Mul<SingleUnit<L, M, T, C, Te, A, Lu>> for f32
{
    type Output = SingleQuantity<L, M, T, C, Te, A, Lu>;

    fn mul(self, rhs: SingleUnit<L, M, T, C, Te, A, Lu>) -> Self::Output {
        SingleQuantity::new(rhs.into(), self)
    }
}

#[derive(Debug, PartialEq)]
struct DynUnit {
    length: i8,
    mass: i8,
    time: i8,
    current: i8,
    temperature: i8,
    amount: i8,
    luminosity: i8,
    system: UnitSystem,
    scale: f32,
    abbreviation: &'static str,
    name: &'static str,
}

impl<L: Integer, M: Integer, T: Integer, C: Integer, Te: Integer, A: Integer, Lu: Integer>
    From<SingleUnit<L, M, T, C, Te, A, Lu>> for DynUnit
{
    fn from(other: SingleUnit<L, M, T, C, Te, A, Lu>) -> Self {
        Self {
            length: L::to_i8(),
            mass: M::to_i8(),
            time: T::to_i8(),
            current: C::to_i8(),
            temperature: Te::to_i8(),
            amount: A::to_i8(),
            luminosity: Lu::to_i8(),
            system: other.system,
            scale: other.scale,
            abbreviation: other.abbreviation,
            name: other.name,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnitSystem {
    length: BaseUnit<Length>,
    mass: BaseUnit<Mass>,
    time: BaseUnit<Time>,
    current: BaseUnit<Current>,
    temperature: BaseUnit<Temperature>,
    amount: BaseUnit<Amount>,
    luminosity: BaseUnit<Luminosity>,
}

macro_rules! unit_kinds {
    ($($units:ident),*) => {
        $(
            #[derive(Debug, PartialEq)]
            pub struct $units;
            impl UnitKind for $units {}
        )*
    };
}

unit_kinds!(Length, Mass, Time, Current, Temperature, Amount, Luminosity);

#[derive(Debug, PartialEq)]
pub struct BaseUnit<Kind: UnitKind> {
    scale_from_si: f32,
    _marker: PhantomData<Kind>,
}

pub trait UnitKind {}

const fn define_base_unit<Kind: UnitKind>(scale: f32) -> BaseUnit<Kind> {
    BaseUnit {
        scale_from_si: scale,
        _marker: PhantomData,
    }
}

const fn define_base_si_unit<Kind: UnitKind>() -> BaseUnit<Kind> {
    define_base_unit(1.)
}

trait UnitFmt {
    fn abbrevation() -> &'static str;
    fn name() -> &'static str;
}

#[allow(non_upper_case_globals)]
pub mod si {
    use super::*;
    use typenum::{N1, N2};

    const METER: BaseUnit<Length> = define_base_si_unit();
    const KILOGRAM: BaseUnit<Mass> = define_base_si_unit();
    const SECOND: BaseUnit<Time> = define_base_si_unit();
    const AMPERE: BaseUnit<Current> = define_base_si_unit();
    const KELVIN: BaseUnit<Temperature> = define_base_si_unit();
    const MOLE: BaseUnit<Amount> = define_base_si_unit();
    const CANDELA: BaseUnit<Luminosity> = define_base_si_unit();

    const SI: UnitSystem = UnitSystem {
        length: METER,
        mass: KILOGRAM,
        time: SECOND,
        current: AMPERE,
        temperature: KELVIN,
        amount: MOLE,
        luminosity: CANDELA,
    };

    macro_rules! kind_to_type {
        (length) => {
            SingleUnit<P1, Z0, Z0, Z0, Z0, Z0, Z0>
        };
        (mass) => {
            SingleUnit<Z0, P1, Z0, Z0, Z0, Z0, Z0>
        };
        (time) => {
            SingleUnit<Z0, Z0, P1, Z0, Z0, Z0, Z0>
        };
        (current) => {
            SingleUnit<Z0, Z0, Z0, P1, Z0, Z0, Z0>
        };
        (temperature) => {
            SingleUnit<Z0, Z0, Z0, Z0, P1, Z0, Z0>
        };
        (amount) => {
            SingleUnit<Z0, Z0, Z0, Z0, Z0, P1, Z0>
        };
        (luminosity) => {
            SingleUnit<Z0, Z0, Z0, Z0, Z0, Z0, P1>
        };
    }

    macro_rules! create_fundamental_unit {
        ($name:ident, $abbreviation:literal, $unit_name:literal, $kind:ident) => {
            #[allow(unused_macros)]
            macro_rules! $name {
                () => {
                    kind_to_type!($kind)
                };
            }
            pub const $name: kind_to_type!($kind) = SingleUnit {
                system: SI,
                scale: 1.,
                abbreviation: $abbreviation,
                name: $unit_name,
                _length_marker: PhantomData,
                _mass_marker: PhantomData,
                _time_marker: PhantomData,
                _current_marker: PhantomData,
                _temperature_marker: PhantomData,
                _amount_marker: PhantomData,
                _luminosity_marker: PhantomData,
            };
        };
    }

    create_fundamental_unit!(m, "m", "meter", length);
    create_fundamental_unit!(kg, "kg", "kilogram", mass);
    create_fundamental_unit!(s, "s", "second", time);
    create_fundamental_unit!(A, "A", "Ampere", current);
    create_fundamental_unit!(K, "K", "Kelvin", temperature);
    create_fundamental_unit!(mol, "mol", "mole", amount);
    create_fundamental_unit!(cd, "cd", "candela", luminosity);

    macro_rules! combine_units_inner {
        ($unit:ident, $power:ty) => {
            <$unit!() as Pow<$power>>::Output
        };
        ($unit1:ident, $power1:ty, $($unit2:ident, $power2:ty),*) => {
            <combine_units!($unit1, $power1) as Mul<combine_units!($($unit2, $power2),*)>>::Output
        };
    }

    macro_rules! combine_units {
        ($unit:ident, $power:ty) => {
            <$unit!() as Pow<$power>>::Output
        };
        ($($unit2:ident, $power2:ty),*) => {
            <combine_units_inner!($($unit2, $power2),*) as ToSingle>::Single
        };
    }

    macro_rules! create_derived_unit {
        ($name:ident, $abbreviation:literal, $unit_name:literal $(, $unit:ident, $power:ty)*) => {
            #[allow(unused_macros)]
            macro_rules! $name {
                () => {combine_units!($($unit, $power),*)};
            }
            pub const $name: combine_units!($($unit, $power),*) = SingleUnit {
                system: SI,
                scale: 1.,
                abbreviation: $abbreviation,
                name: $unit_name,
                _length_marker: PhantomData,
                _mass_marker: PhantomData,
                _time_marker: PhantomData,
                _current_marker: PhantomData,
                _temperature_marker: PhantomData,
                _amount_marker: PhantomData,
                _luminosity_marker: PhantomData,
            };
        };
    }

    create_derived_unit!(N, "N", "Newton", kg, P1, m, P1, s, N2);
    create_derived_unit!(J, "J", "Joule", N, P1, m, P1);
    create_derived_unit!(W, "W", "Watt", J, P1, s, N1);
}
