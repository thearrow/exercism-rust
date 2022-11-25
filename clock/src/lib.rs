use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (new_hours, new_minutes) = Self::compute_rollover(hours, minutes);
        Self {
            hours: new_hours,
            minutes: new_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }

    fn compute_rollover(hours: i32, minutes: i32) -> (i32, i32) {
        let mut new_hours = hours;
        let mut new_minutes = minutes;

        if minutes < 0 || minutes >= 60 {
            let more_hours = minutes / 60;
            let rem = minutes.rem_euclid(60);
            new_hours += more_hours;
            if minutes < 0 && rem != 0 {
                new_hours -= 1;
            }
            new_minutes = rem;
        }
        if new_hours < 0 || new_hours >= 24 {
            let rem = new_hours.rem_euclid(24);
            new_hours = rem;
        }

        (new_hours, new_minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
