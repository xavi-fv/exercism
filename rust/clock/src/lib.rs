use std::fmt::{Display, Formatter};

const MIN_IN_HOUR: i32 = 60;
const TOTAL_MIN: i32 = MIN_IN_HOUR * 24;


#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { minutes: ((hours * MIN_IN_HOUR + minutes) % TOTAL_MIN + TOTAL_MIN) % TOTAL_MIN }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:02}:{:02}", self.minutes / MIN_IN_HOUR, self.minutes % MIN_IN_HOUR))
    }
}