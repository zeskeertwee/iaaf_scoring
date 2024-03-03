use std::time::Duration;
use crate::event::OutdoorEvent;
use crate::gender::Gender;
use crate::methods::{iaaf_2017_tables_outdoor, ScoringMethod};
use crate::preformance::Preformance;

#[test]
fn test_out_of_table_preformance() {
    let mut tables = iaaf_2017_tables_outdoor::Iaaf2017Tables::new();

    assert_eq!(tables.score(OutdoorEvent::Track200m, Gender::Male, Preformance::Time(Duration::from_millis(26150))).unwrap(), 444);

    // falls in between two table entries
    assert_eq!(tables.score(OutdoorEvent::Track100m, Gender::Male, Preformance::Time(Duration::from_millis(15340))).unwrap(), 67);

    // also falls off the table
    assert_eq!(tables.score(OutdoorEvent::HighJump, Gender::Female, Preformance::Distance(2.20)).unwrap(), 1399);

    // falls off the table
    assert_eq!(tables.score(OutdoorEvent::Track100m, Gender::Male, Preformance::Time(Duration::from_millis(9400))).unwrap(), 1400);
    assert_eq!(tables.score(OutdoorEvent::Track100m, Gender::Female, Preformance::Time(Duration::from_millis(21700))).unwrap(), 0);
}
