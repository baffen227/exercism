use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = if hours >= 0 { hours } else {
            (hours % 24) + 24
        };

        let total_minutes = hours * 60 + minutes;
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;

        Clock {
            hours: if hours >= 24 { hours % 24 } else { hours },
            minutes,
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