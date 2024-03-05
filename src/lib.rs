pub mod error;
pub mod event;
pub mod gender;
pub mod methods;
pub mod preformance;

#[cfg(test)]
mod tests;
trait Codename {
    fn to_codename(&self) -> &str;
    fn from_codename(code: &str) -> Option<Self> where Self: Sized;
}
