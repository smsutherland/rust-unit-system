use crate::quantity::SingleQuantity;

use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Div, Mul};

#[derive(Debug)]
pub struct CompositeUnit {
    numerator_units: Vec<(SingleUnit, u8)>,
    denominator_units: Vec<(SingleUnit, u8)>,
}

impl Mul for CompositeUnit {
    type Output = CompositeUnit;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        self.numerator_units.append(&mut rhs.numerator_units);
        self.denominator_units.append(&mut rhs.denominator_units);
        CompositeUnit {
            numerator_units: self.numerator_units,
            denominator_units: self.denominator_units,
        }
    }
}

impl Mul<SingleUnit> for CompositeUnit {
    type Output = CompositeUnit;

    fn mul(mut self, rhs: SingleUnit) -> Self::Output {
        for (unit, power) in self.numerator_units.iter_mut() {
            if *unit == rhs {
                *power += 1;
                return self;
            }
        }
        for (i, (unit, power)) in self.denominator_units.iter_mut().enumerate() {
            if *unit == rhs {
                if *power == 1 {
                    self.denominator_units.swap_remove(i);
                } else {
                    *power -= 1;
                }
                return self;
            }
        }
        self.numerator_units.push((rhs, 1));
        self
    }
}

impl From<SingleUnit> for CompositeUnit {
    fn from(other: SingleUnit) -> Self {
        CompositeUnit {
            numerator_units: vec![(other, 1)],
            denominator_units: Vec::new(),
        }
    }
}

impl Display for CompositeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, (unit, power)) in self.numerator_units.iter().enumerate() {
            match (i, power) {
                (_, 0) => {}
                (0, 1) => write!(f, "{}", unit.abbreviation)?,
                (0, _) => write!(f, "{}^{}", unit.abbreviation, power)?,
                (_, 1) => write!(f, " {}", unit.abbreviation)?,
                (_, _) => write!(f, " {}^{}", unit.abbreviation, power)?,
            }
        }
        for (unit, power) in &self.denominator_units {
            if *power != 0 {
                write!(f, " {}^-{}", unit.abbreviation(), power)?
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct SingleUnit {
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

impl SingleUnit {
    pub fn abbreviation(&self) -> &'static str {
        self.abbreviation
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

impl Mul for SingleUnit {
    type Output = CompositeUnit;

    fn mul(self, rhs: Self) -> Self::Output {
        CompositeUnit {
            numerator_units: vec![(self, 1), (rhs, 1)],
            denominator_units: Vec::new(),
        }
    }
}

impl Div for SingleUnit {
    type Output = CompositeUnit;

    fn div(self, rhs: Self) -> Self::Output {
        CompositeUnit {
            numerator_units: vec![(self, 1)],
            denominator_units: vec![(rhs, 1)],
        }
    }
}

impl Mul<f32> for SingleUnit {
    type Output = SingleQuantity;

    fn mul(self, rhs: f32) -> Self::Output {
        SingleQuantity::new(self.into(), rhs)
    }
}

impl Mul<SingleUnit> for f32 {
    type Output = SingleQuantity;

    fn mul(self, rhs: SingleUnit) -> Self::Output {
        SingleQuantity::new(rhs.into(), self)
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

    pub const METER: BaseUnit<Length> = define_base_si_unit();
    pub const KILOGRAM: BaseUnit<Mass> = define_base_si_unit();
    pub const SECOND: BaseUnit<Time> = define_base_si_unit();
    pub const AMPERE: BaseUnit<Current> = define_base_si_unit();
    pub const KELVIN: BaseUnit<Temperature> = define_base_si_unit();
    pub const MOLE: BaseUnit<Amount> = define_base_si_unit();
    pub const CANDELA: BaseUnit<Luminosity> = define_base_si_unit();

    pub const SI: UnitSystem = UnitSystem {
        length: METER,
        mass: KILOGRAM,
        time: SECOND,
        current: AMPERE,
        temperature: KELVIN,
        amount: MOLE,
        luminosity: CANDELA,
    };

    macro_rules! parse_unit_kind {
        (length, length) => {
            1
        };
        (mass, mass) => {
            1
        };
        (time, time) => {
            1
        };
        (current, current) => {
            1
        };
        (temperature, temperature) => {
            1
        };
        (amount, amount) => {
            1
        };
        (luminosity, luminosity) => {
            1
        };
        ($i1:ident, $i2:ident) => {
            0
        };
    }

    macro_rules! create_fundamental_unit {
        ($abbreviation:literal, $unit_name:literal, $kind:ident) => {
            SingleUnit {
                length: parse_unit_kind!(length, $kind),
                mass: parse_unit_kind!(mass, $kind),
                time: parse_unit_kind!(time, $kind),
                current: parse_unit_kind!(current, $kind),
                temperature: parse_unit_kind!(temperature, $kind),
                amount: parse_unit_kind!(amount, $kind),
                luminosity: parse_unit_kind!(luminosity, $kind),
                system: SI,
                scale: 1.,
                abbreviation: $abbreviation,
                name: $unit_name,
            }
        };
    }

    pub const m: SingleUnit = create_fundamental_unit!("m", "meter", length);
    pub const kg: SingleUnit = create_fundamental_unit!("kg", "kilogram", mass);
    pub const s: SingleUnit = create_fundamental_unit!("s", "second", time);
    pub const A: SingleUnit = create_fundamental_unit!("A", "Ampere", current);
    pub const K: SingleUnit = create_fundamental_unit!("K", "Kelvin", temperature);
    pub const mol: SingleUnit = create_fundamental_unit!("mol", "mole", amount);
    pub const cd: SingleUnit = create_fundamental_unit!("cd", "candela", luminosity);
}

#[test]
fn feature() {
    let len = 5. * si::m;
    println!("{len:?}");
}
