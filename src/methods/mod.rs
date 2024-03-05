use std::collections::HashMap;
use std::io::{Cursor, Read};
use rust_embed::RustEmbed;
use crate::event::{EventCompareType, OutdoorEvent};
use crate::gender::Gender;
use crate::preformance::Preformance;
use lazy_static::lazy_static;
use log::{error, info, trace, warn};
use serde::Deserialize;
use tar::Archive;
use crate::Codename;

pub mod iaaf_2017_tables_outdoor;

#[derive(Deserialize)]
struct TableEntry {
    performance: f64,
    points: u64,
}

pub trait ScoringMethod<E> {
    /// returns the score that corresponds with the performance
    fn score(
        &mut self,
        event: E,
        gender: Gender,
        performance: Preformance,
    ) -> Option<u64>;
}

#[derive(RustEmbed)]
#[folder = "resources/"]
#[include = "*.tar.lzma"]
#[exclude = "*.csv"]
struct EmbeddedLZMATables;

lazy_static! {
    pub static ref TABLES: EmbeddedTables = EmbeddedTables::init();
}

pub struct EmbeddedTables {
    tables: HashMap<String, Vec<TableEntry>>,
}

impl EmbeddedTables {
    fn init() -> Self {
        let mut tables = HashMap::new();
        trace!("Listing embedded LZMA tables");
        for i in EmbeddedLZMATables::iter() {
            trace!("- {}", i);

            let file = EmbeddedLZMATables::get(&i).unwrap();
            let mut buf = Vec::with_capacity(file.data.len());
            lzma_rs::lzma_decompress(&mut Cursor::new(file.data), &mut buf).unwrap();

            let mut archive = Archive::new(Cursor::new(buf));
            for entry in archive.entries().unwrap() {
                let mut file = entry.unwrap();
                let mut buf = Vec::new();
                file.read_to_end(&mut buf).unwrap();

                let path = file.path().unwrap().file_name().unwrap().to_string_lossy().to_string();
                trace!("Reading {}", path);
                let event = path.split("-").last().unwrap().replace(".csv", "");
                let cmp_type = {
                    if path.to_lowercase().contains("indoor") {
                        warn!("Unimplemented indoor events");
                        continue;
                    } else if path.to_lowercase().contains("outdoor") {
                        match OutdoorEvent::from_codename(event.trim()) {
                            Some(e) => e.compare_type(),
                            None => {
                                error!("Invalid outdoor event {}", event.trim());
                                continue;
                            }
                        }
                    } else {
                        error!("Neither indoor nor outdoor table: {}", path);
                        continue;
                    }
                };

                let table = parse_table(buf, cmp_type);

                tables.insert(path, table);
            }
        }

        Self {
            tables
        }
    }

    fn read_table<T, F: Fn(&Vec<TableEntry>) -> T>(table: &str, func: F) -> Option<T> {
        TABLES.tables.get(table).map(|v| func(v))
    }
}

fn parse_table(data: Vec<u8>, k: EventCompareType) -> Vec<TableEntry> {
    let mut data: Vec<TableEntry> = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_reader(Cursor::new(data))
        .records()
        .filter(|i| i.is_ok())
        .map(|i| i.unwrap())
        .map(|i| TableEntry {
            performance: i[0].parse().unwrap(),
            points: i[1].parse().unwrap(),
        })
        .collect();

    match k {
        EventCompareType::SmallerIsBetter => data.sort_by(|a, b| a.performance.total_cmp(&b.performance)),
        EventCompareType::GreaterIsBetter => data.sort_by(|a, b| b.performance.total_cmp(&a.performance))
    };

    data
}