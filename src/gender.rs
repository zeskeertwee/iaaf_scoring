use crate::ToCodename;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Gender {
    Male,
    Female,
}

impl ToCodename for Gender {
    fn to_codename(&self) -> &str {
        match self {
            Self::Female => "FEMALE",
            Self::Male => "MALE",
        }
    }
}
