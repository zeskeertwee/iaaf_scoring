use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(PartialEq, Copy, Clone)]
pub enum Preformance {
    /// Preformance is measured in distance (m) (HJ, LJ, etc.)
    Distance(f64),

    /// Preformance is measured in time (s) (100m, 400mH, Marathon, etc.)
    Time(Duration),

    /// Preformance is measured in points (Decathlon, Heptathlon)
    Points(u16),
}

impl Preformance {
    /// Returns the preformance as a float
    /// - For distance this is the distance in meters
    /// - For time this is the time in seconds
    /// - For points this is the amount of points
    pub fn to_number(&self) -> f64 {
        match self {
            Preformance::Distance(distance) => *distance,
            Preformance::Time(duration) => duration.as_secs_f64(),
            Preformance::Points(points) => *points as f64,
        }
    }
}

impl Display for Preformance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Preformance::Distance(dist) => write!(f, "{:.3} m", dist),
            Preformance::Points(points) => write!(f, "{} pts", points),
            Preformance::Time(dur) => write!(f, "{:.3} s", dur.as_secs_f64()),
        }
    }
}
