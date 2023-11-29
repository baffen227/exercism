// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600.0
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
        // Mercury: orbital period 0.2408467 Earth years
        // 1 Earch year = 1 / 0.2408467 Mercury years
        (d.seconds as f64 / 31557600.0) / 0.2408467
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        // Venus: orbital period 0.61519726 Earth years
        // 1 Earch year = 1 / 0.61519726 Venus years
        (d.seconds as f64 / 31557600.0) / 0.61519726
    }
}
impl Planet for Earth {}
impl Planet for Mars {}
impl Planet for Jupiter {}
impl Planet for Saturn {}
impl Planet for Uranus {}
impl Planet for Neptune {}
