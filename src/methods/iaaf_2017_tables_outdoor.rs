use crate::event::OutdoorEvent;
use crate::gender::Gender;
use crate::methods::ScoringMethod;
use crate::preformance::Preformance;
use crate::ToCodename;
use log::{error, info, trace};
use rust_embed::{EmbeddedFile, RustEmbed};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Cursor;
use std::time::Instant;
use strum::IntoEnumIterator;

#[derive(Deserialize)]
struct TableEntry {
    performance: f64,
    points: u64,
}

#[derive(RustEmbed)]
#[folder = "resources/iaaf_2017_tables"]
struct Tables;

pub struct Iaaf2017Tables {
    loaded_tables: HashMap<(OutdoorEvent, Gender), Vec<TableEntry>>,
}

impl ScoringMethod for Iaaf2017Tables {
    fn score(
        &mut self,
        event: OutdoorEvent,
        gender: Gender,
        performance: Preformance,
    ) -> Option<u64> {
        if !event.exists_for_gender(&gender) {
            return None;
        }

        match self.loaded_tables.get(&(event, gender)) {
            Some(table) => table
                .iter()
                .find(|i| i.performance == performance.to_number())
                .map(|i| i.points),
            None => {
                self.load_table(event, gender);
                self.score(event, gender, performance)
            }
        }
    }
}

impl Iaaf2017Tables {
    pub fn new() -> Self {
        trace!("Listing embedded files:");
        for i in Tables::iter() {
            trace!("{}", i);
        }

        Self {
            loaded_tables: HashMap::new(),
        }
    }

    pub fn load_all_tables(&mut self) {
        info!("Loading all tables");
        let start = Instant::now();
        for event in OutdoorEvent::iter() {
            self.load_table(event, Gender::Female);
            self.load_table(event, Gender::Male);
        }
        let end = start.elapsed().as_secs_f64();
        info!("Finished loading all tables (took {:.2} ms)", end * 1000.0);
    }

    fn load_table(&mut self, event: OutdoorEvent, gender: Gender) {
        if !event.exists_for_gender(&gender) {
            trace!("Event {:?} does not exist for {:?}", event, gender);
            return;
        }

        if self.loaded_tables.get(&(event, gender)).is_some() {
            trace!("Table for {:?} - {:?} already loaded", event, gender);
            return;
        }

        let fname = Self::format_name(event, gender);
        info!("Loading table '{}'", fname);
        match Tables::get(&fname) {
            Some(data) => {
                let data: Vec<TableEntry> = csv::ReaderBuilder::new()
                    .delimiter(b',')
                    .has_headers(true)
                    .from_reader(Cursor::new(data.data))
                    .records()
                    .filter(|i| i.is_ok())
                    .map(|i| i.unwrap())
                    .map(|i| TableEntry {
                        performance: i[0].parse().unwrap(),
                        points: i[1].parse().unwrap(),
                    })
                    .collect();

                self.loaded_tables.insert((event, gender), data);
                info!(
                    "Loading table '{}' finished, {} tables now loaded",
                    fname,
                    self.loaded_tables.len()
                );
            }
            None => panic!("Missing table '{}'!", fname),
        }
    }

    fn format_name(event: OutdoorEvent, gender: Gender) -> String {
        format!(
            "Table Outdoor 2017 - {} - {}.csv",
            gender.to_codename(),
            event.to_codename()
        )
    }
}
