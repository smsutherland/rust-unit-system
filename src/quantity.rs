use crate::unit::{CompositeUnit, SingleUnit};
use std::fmt::Display;
use std::ops::{Add, Mul};
use typenum::{Integer, Sum};

#[derive(Debug)]
pub struct SingleQuantity<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
where
    Length: Integer,
    Mass: Integer,
    Time: Integer,
    Current: Integer,
    Temperature: Integer,
    Amount: Integer,
    Luminosity: Integer,
{
    unit: CompositeUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>,
    scalar: f32,
}

impl<
        Length: Integer,
        Mass: Integer,
        Time: Integer,
        Current: Integer,
        Temperature: Integer,
        Amount: Integer,
        Luminosity: Integer,
    > SingleQuantity<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
{
    pub fn new(
        unit: CompositeUnit<Length, Mass, Time, Current, Temperature, Amount, Luminosity>,
        scalar: f32,
    ) -> Self {
        Self { unit, scalar }
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
    > Mul<SingleQuantity<Length1, Mass1, Time1, Current1, Temperature1, Amount1, Luminosity1>>
    for SingleQuantity<Length2, Mass2, Time2, Current2, Temperature2, Amount2, Luminosity2>
where
    Sum<Length1, Length2>: Integer,
    Sum<Mass1, Mass2>: Integer,
    Sum<Time1, Time2>: Integer,
    Sum<Current1, Current2>: Integer,
    Sum<Temperature1, Temperature2>: Integer,
    Sum<Amount1, Amount2>: Integer,
    Sum<Luminosity1, Luminosity2>: Integer,
{
    type Output = SingleQuantity<
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
        rhs: SingleQuantity<Length1, Mass1, Time1, Current1, Temperature1, Amount1, Luminosity1>,
    ) -> Self::Output {
        Self::Output::new(self.unit * rhs.unit, self.scalar * rhs.scalar)
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
    for SingleQuantity<Length2, Mass2, Time2, Current2, Temperature2, Amount2, Luminosity2>
where
    Sum<Length1, Length2>: Integer,
    Sum<Mass1, Mass2>: Integer,
    Sum<Time1, Time2>: Integer,
    Sum<Current1, Current2>: Integer,
    Sum<Temperature1, Temperature2>: Integer,
    Sum<Amount1, Amount2>: Integer,
    Sum<Luminosity1, Luminosity2>: Integer,
{
    type Output = SingleQuantity<
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
        Self::Output {
            unit: self.unit * rhs,
            scalar: self.scalar,
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
    > Display for SingleQuantity<Length, Mass, Time, Current, Temperature, Amount, Luminosity>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.scalar, self.unit)
    }
}
