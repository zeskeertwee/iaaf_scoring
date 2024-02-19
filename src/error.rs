use serde::ser;

pub enum IaafScoringError {
    Serde(String),
}
