pub mod error;
pub mod event;
pub mod gender;
pub mod methods;
pub mod preformance;

trait ToCodename {
    fn to_codename(&self) -> &str;
}
