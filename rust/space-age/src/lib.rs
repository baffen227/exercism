const ORBITAL_PERIOD_OF_EARTH_IN_SECONDS: f64 = 31557600.0;
const ORBITAL_PERIOD_OF_MERCURY_IN_EARTH_YEARS: f64 = 0.2408467;
const ORBITAL_PERIOD_OF_VENUS_IN_EARTH_YEARS: f64 = 0.61519726;
const ORBITAL_PERIOD_OF_MARS_IN_EARTH_YEARS: f64 = 1.8808158;
const ORBITAL_PERIOD_OF_JUPITER_IN_EARTH_YEARS: f64 = 11.862615;
const ORBITAL_PERIOD_OF_SATURN_IN_EARTH_YEARS: f64 = 29.447498;
const ORBITAL_PERIOD_OF_URANUS_IN_EARTH_YEARS: f64 = 84.016846;
const ORBITAL_PERIOD_OF_NEPTUNE_IN_EARTH_YEARS: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { earth_years: s as f64 / ORBITAL_PERIOD_OF_EARTH_IN_SECONDS }
    }
}

pub trait Planet {
    // Default implementation for Earth
    fn years_during(d: &Duration) -> f64 {
        d.earth_years
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_OF_MERCURY_IN_EARTH_YEARS
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_OF_VENUS_IN_EARTH_YEARS
    }
}

impl Planet for Earth {}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_OF_MARS_IN_EARTH_YEARS
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_OF_JUPITER_IN_EARTH_YEARS
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_OF_SATURN_IN_EARTH_YEARS
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_OF_URANUS_IN_EARTH_YEARS
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / ORBITAL_PERIOD_OF_NEPTUNE_IN_EARTH_YEARS
    }
}
