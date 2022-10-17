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

impl<
        Length: Integer,
        Mass: Integer,
        Time: Integer,
        Current: Integer,
        Temperature: Integer,
        Amount: Integer,
        Luminosity: Integer,
    > ToSingle for CompositeUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
{
    type Single = SingleUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>;
}

impl<
        Length1: Integer + Add<Length2>,
        Mass1: Integer + Add<Mass2>,
        Time1: Integer + Add<Time2>,
        Current1: Integer + Add<Current2>,
        Temperature1: Integer + Add<Temperature2>,
        Amount1: Integer + Add<Amount2>,
        Luminosity1: Integer + Add<Luminosity2>,
        Length2: Integer,
        Mass2: Integer,
        Time2: Integer,
        Current2: Integer,
        Temperature2: Integer,
        Amount2: Integer,
        Luminosity2: Integer,
    > Mul<CompositeUnit<Length1, Mass1, Time1, Current1, Temperature1, Amount1, Luminosity1>>
    for CompositeUnit<Length2, Mass2, Time2, Current2, Temperature2, Amount2, Luminosity2>
where
    Sum<Length1, Length2>: Integer,
    Sum<Mass1, Mass2>: Integer,
    Sum<Time1, Time2>: Integer,
    Sum<Current1, Current2>: Integer,
    Sum<Temperature1, Temperature2>: Integer,
    Sum<Amount1, Amount2>: Integer,
    Sum<Luminosity1, Luminosity2>: Integer,
{
    type Output = CompositeUnit<
        Sum<Length1, Length2>,
        Sum<Mass1, Mass2>,
        Sum<Time1, Time2>,
        Sum<Current1, Current2>,
        Sum<Temperature1, Temperature2>,
        Sum<Amount1, Amount2>,
        Sum<Luminosity1, Luminosity2>,
    >;

    fn mul(
        mut self,
        mut rhs: CompositeUnit<Length1, Mass1, Time1, Current1, Temperature1, Amount1, Luminosity1>,
    ) -> Self::Output {
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
        Length1: Integer + Add<Length2>,
        Mass1: Integer + Add<Mass2>,
        Time1: Integer + Add<Time2>,
        Current1: Integer + Add<Current2>,
        Temperature1: Integer + Add<Temperature2>,
        Amount1: Integer + Add<Amount2>,
        Luminosity1: Integer + Add<Luminosity2>,
        Length2: Integer,
        Mass2: Integer,
        Time2: Integer,
        Current2: Integer,
        Temperature2: Integer,
        Amount2: Integer,
        Luminosity2: Integer,
    > Mul<SingleUnit<Length1, Mass1, Time1, Current1, Temperature1, Amount1, Luminosity1>>
    for CompositeUnit<Length2, Mass2, Time2, Current2, Temperature2, Amount2, Luminosity2>
where
    Sum<Length1, Length2>: Integer,
    Sum<Mass1, Mass2>: Integer,
    Sum<Time1, Time2>: Integer,
    Sum<Current1, Current2>: Integer,
    Sum<Temperature1, Temperature2>: Integer,
    Sum<Amount1, Amount2>: Integer,
    Sum<Luminosity1, Luminosity2>: Integer,
{
    type Output = CompositeUnit<
        Sum<Length1, Length2>,
        Sum<Mass1, Mass2>,
        Sum<Time1, Time2>,
        Sum<Current1, Current2>,
        Sum<Temperature1, Temperature2>,
        Sum<Amount1, Amount2>,
        Sum<Luminosity1, Luminosity2>,
    >;

    fn mul(
        mut self,
        rhs: SingleUnit<Length1, Mass1, Time1, Current1, Temperature1, Amount1, Luminosity1>,
    ) -> Self::Output {
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

impl<
        Length: Integer,
        Mass: Integer,
        Time: Integer,
        Current: Integer,
        Temperature: Integer,
        Amount: Integer,
        Luminosity: Integer,
    > From<SingleUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>>
    for CompositeUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
{
    fn from(
        other: SingleUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>,
    ) -> Self {
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

impl<
        Length: Integer,
        Mass: Integer,
        Time: Integer,
        Current: Integer,
        Temperature: Integer,
        Amount: Integer,
        Luminosity: Integer,
    > Display for CompositeUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
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

impl<
        Length: Integer,
        Mass: Integer,
        Time: Integer,
        Current: Integer,
        Temperature: Integer,
        Amount: Integer,
        Luminosity: Integer,
    > std::fmt::Debug for SingleUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.abbreviation)
    }
}

impl<
        Length: Integer + Mul<Power>,
        Mass: Integer + Mul<Power>,
        Time: Integer + Mul<Power>,
        Current: Integer + Mul<Power>,
        Temperature: Integer + Mul<Power>,
        Amount: Integer + Mul<Power>,
        Luminosity: Integer + Mul<Power>,
        Power: Integer,
    > Pow<Power> for SingleUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
where
    Prod<Length, Power>: Integer,
    Prod<Mass, Power>: Integer,
    Prod<Time, Power>: Integer,
    Prod<Current, Power>: Integer,
    Prod<Temperature, Power>: Integer,
    Prod<Amount, Power>: Integer,
    Prod<Luminosity, Power>: Integer,
{
    type Output = SingleUnit<
        Prod<Length, Power>,
        Prod<Mass, Power>,
        Prod<Time, Power>,
        Prod<Current, Power>,
        Prod<Temperature, Power>,
        Prod<Amount, Power>,
        Prod<Luminosity, Power>,
    >;
}

impl<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
    SingleUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
where
    Length: Integer,
    Mass: Integer,
    Current: Integer,
    Temperature: Integer,
    Amount: Integer,
    Luminosity: Integer,
{
    pub fn abbreviation(&self) -> &'static str {
        self.abbreviation
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl<
        Length1: Integer + Add<Length2>,
        Mass1: Integer + Add<Mass2>,
        Time1: Integer + Add<Time2>,
        Current1: Integer + Add<Current2>,
        Temperature1: Integer + Add<Temperature2>,
        Amount1: Integer + Add<Amount2>,
        Luminosity1: Integer + Add<Luminosity2>,
        Length2: Integer,
        Mass2: Integer,
        Time2: Integer,
        Current2: Integer,
        Temperature2: Integer,
        Amount2: Integer,
        Luminosity2: Integer,
    > Mul<SingleUnit<Length1, Mass1, Time1, Current1, Temperature1, Amount1, Luminosity1>>
    for SingleUnit<Length2, Mass2, Time2, Current2, Temperature2, Amount2, Luminosity2>
where
    Sum<Length1, Length2>: Integer,
    Sum<Mass1, Mass2>: Integer,
    Sum<Time1, Time2>: Integer,
    Sum<Current1, Current2>: Integer,
    Sum<Temperature1, Temperature2>: Integer,
    Sum<Amount1, Amount2>: Integer,
    Sum<Luminosity1, Luminosity2>: Integer,
{
    type Output = CompositeUnit<
        Sum<Length1, Length2>,
        Sum<Mass1, Mass2>,
        Sum<Time1, Time2>,
        Sum<Current1, Current2>,
        Sum<Temperature1, Temperature2>,
        Sum<Amount1, Amount2>,
        Sum<Luminosity1, Luminosity2>,
    >;

    fn mul(
        self,
        rhs: SingleUnit<Length1, Mass1, Time1, Current1, Temperature1, Amount1, Luminosity1>,
    ) -> Self::Output {
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
        Length1: Integer,
        Mass1: Integer,
        Time1: Integer,
        Current1: Integer,
        Temperature1: Integer,
        Amount1: Integer,
        Luminosity1: Integer,
        Length2: Integer + Add<Length1>,
        Mass2: Integer + Add<Mass1>,
        Time2: Integer + Add<Time1>,
        Current2: Integer + Add<Current1>,
        Temperature2: Integer + Add<Temperature1>,
        Amount2: Integer + Add<Amount1>,
        Luminosity2: Integer + Add<Luminosity1>,
    > Mul<CompositeUnit<Length1, Mass1, Time1, Current1, Temperature1, Amount1, Luminosity1>>
    for SingleUnit<Length2, Mass2, Time2, Current2, Temperature2, Amount2, Luminosity2>
where
    Sum<Length2, Length1>: Integer,
    Sum<Mass2, Mass1>: Integer,
    Sum<Time2, Time1>: Integer,
    Sum<Current2, Current1>: Integer,
    Sum<Temperature2, Temperature1>: Integer,
    Sum<Amount2, Amount1>: Integer,
    Sum<Luminosity2, Luminosity1>: Integer,
{
    type Output = CompositeUnit<
        Sum<Length2, Length1>,
        Sum<Mass2, Mass1>,
        Sum<Time2, Time1>,
        Sum<Current2, Current1>,
        Sum<Temperature2, Temperature1>,
        Sum<Amount2, Amount1>,
        Sum<Luminosity2, Luminosity1>,
    >;

    fn mul(
        self,
        rhs: CompositeUnit<Length1, Mass1, Time1, Current1, Temperature1, Amount1, Luminosity1>,
    ) -> Self::Output {
        rhs * self
    }
}

impl<
        Length: Integer,
        Mass: Integer,
        Time: Integer,
        Current: Integer,
        Temperature: Integer,
        Amount: Integer,
        Luminosity: Integer,
    > Div for SingleUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
{
    type Output = CompositeUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>;

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

impl<
        Length: Integer,
        Mass: Integer,
        Time: Integer,
        Current: Integer,
        Temperature: Integer,
        Amount: Integer,
        Luminosity: Integer,
    > Mul<f32> for SingleUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
{
    type Output = SingleQuantity<Length, Mass, Time, Current, Temperature, Amount, Luminosity>;

    fn mul(self, rhs: f32) -> Self::Output {
        SingleQuantity::new(self.into(), rhs)
    }
}

impl<
        Length: Integer,
        Mass: Integer,
        Time: Integer,
        Current: Integer,
        Temperature: Integer,
        Amount: Integer,
        Luminosity: Integer,
    > Mul<SingleUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>> for f32
{
    type Output = SingleQuantity<Length, Mass, Time, Current, Temperature, Amount, Luminosity>;

    fn mul(
        self,
        rhs: SingleUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>,
    ) -> Self::Output {
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

impl<
        Length: Integer,
        Mass: Integer,
        Time: Integer,
        Current: Integer,
        Temperature: Integer,
        Amount: Integer,
        Luminosity: Integer,
    > From<SingleUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>> for DynUnit
{
    fn from(
        other: SingleUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>,
    ) -> Self {
        Self {
            length: Length::to_i8(),
            mass: Mass::to_i8(),
            time: Time::to_i8(),
            current: Current::to_i8(),
            temperature: Temperature::to_i8(),
            amount: Amount::to_i8(),
            luminosity: Luminosity::to_i8(),
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
