use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // rounded in a day (24 * 60 = 1440 minutes)
        let total = (hours * 60 + minutes) % 1440;
        // start from today or yesterday
        let total = if total >= 0 { total } else { 1440 + total };

        Clock {
            hours: total / 60,
            minutes: total % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        todo!("Add {minutes} minutes to existing Clock time");
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}