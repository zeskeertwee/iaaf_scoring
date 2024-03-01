pub mod error;
pub mod event;
pub mod gender;
pub mod methods;
pub mod preformance;

#[cfg(test)]
mod tests;
trait ToCodename {
    fn to_codename(&self) -> &str;
}
