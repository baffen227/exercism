use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    const A_DAY_IN_MINUTES: i32 = 24 * 60;

    pub fn new(hours: i32, minutes: i32) -> Self {
        let total = (hours * 60 + minutes) % Self::A_DAY_IN_MINUTES;
        // start from today or yesterday
        let total = if total >= 0 { total } else { Self::A_DAY_IN_MINUTES + total };

        Clock {
            hours: total / 60,
            minutes: total % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, minutes+self.minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}