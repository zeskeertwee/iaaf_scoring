pub mod outdoor;

pub use outdoor::OutdoorEvent;

pub enum EventCompareType {
    GreaterIsBetter,
    SmallerIsBetter,
}