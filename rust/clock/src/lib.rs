use std::fmt;
const MINUTES_PER_DAY: i32 = 1440;
const MINUTES_PER_HOUR: i32 = 60;

#[derive(PartialEq, Debug)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: (hours * MINUTES_PER_HOUR + minutes).rem_euclid(MINUTES_PER_DAY),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "{:02}:{:02}",
            self.minutes / MINUTES_PER_HOUR,
            self.minutes % MINUTES_PER_HOUR)
    }
}