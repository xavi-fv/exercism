use std::ops::Div;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64 / 31557600.)
    }
}

impl Div<f64> for &Duration {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        self.0 / rhs
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! impl_planet {
    ( $planet:ident, $orbital:expr ) => {
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                d / ($orbital)
            }
        }
    }
}

impl_planet!(Mercury, 0.2408467);
impl_planet!(Venus, 0.61519726);
impl_planet!(Earth, 1.0);
impl_planet!(Mars, 1.8808158);
impl_planet!(Jupiter, 11.862615);
impl_planet!(Saturn, 29.447498);
impl_planet!(Uranus, 84.016846);
impl_planet!(Neptune, 164.79132);

