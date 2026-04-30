use core::fmt::Display;
use core::ops::AddAssign;
use core::ops::Rem as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HourMinute {
    pub hours: u8,
    pub minutes: u8,
}

impl AddAssign for HourMinute {
    fn add_assign(&mut self, rhs: Self) {
        let minutes = (self.minutes + rhs.minutes).rem(60);
        let add_hours = (self.minutes + rhs.minutes).div_euclid(60);

        self.minutes = minutes;
        self.hours = (self.hours + rhs.hours + add_hours).rem(24);
    }
}

impl Display for HourMinute {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
