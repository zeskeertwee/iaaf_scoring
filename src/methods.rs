use crate::event::OutdoorEvent;
use crate::gender::Gender;
use crate::preformance::Preformance;

pub mod iaaf_2017_tables_outdoor;

pub trait ScoringMethod {
    /// returns the score that corresponds with the performance
    fn score(
        &mut self,
        event: OutdoorEvent,
        gender: Gender,
        performance: Preformance,
    ) -> Option<u64>;
}
