use crate::event::{OutdoorEvent, EventCompareType};
use crate::gender::Gender;
use crate::methods::{ScoringMethod, EmbeddedTables, TableEntry};
use crate::preformance::Preformance;
use crate::Codename;
use log::{error, info, trace};
use rust_embed::{EmbeddedFile, RustEmbed};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Cursor;
use std::time::Instant;
use strum::IntoEnumIterator;

pub struct Iaaf2017Tables;

impl ScoringMethod<OutdoorEvent> for Iaaf2017Tables {
    fn score(
        &mut self,
        event: OutdoorEvent,
        gender: Gender,
        performance: Preformance,
    ) -> Option<u64> {
        if !event.exists_for_gender(&gender) {
            return None;
        }

        EmbeddedTables::read_table(&Self::format_name(event, gender), |v| {
            for (idx, perf) in v.iter().enumerate() {
                match event.compare_type() {
                    EventCompareType::SmallerIsBetter => {
                        if perf.performance >= performance.to_number() {
                            return perf.points
                        }
                    },
                    EventCompareType::GreaterIsBetter => {
                        if perf.performance <= performance.to_number() {
                            return perf.points
                        }
                    }
                }
            }

            // Falls outside of the table
            0
        })
    }
}

impl Iaaf2017Tables {
    pub fn new() -> Self {
        Self {}
    }

    fn format_name(event: OutdoorEvent, gender: Gender) -> String {
        format!(
            "Table Outdoor 2017 - {} - {}.csv",
            gender.to_codename(),
            event.to_codename()
        )
    }
}
