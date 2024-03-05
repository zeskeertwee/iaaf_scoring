use crate::Codename;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Gender {
    Male,
    Female,
}

impl Codename for Gender {
    fn to_codename(&self) -> &str {
        match self {
            Self::Female => "FEMALE",
            Self::Male => "MALE",
        }
    }

    fn from_codename(code: &str) -> Option<Self> {
        match code {
            "FEMALE" | "W" => Some(Self::Female),
            "MALE" | "M" => Some(Self::Male),
            _ => None
        }
    }
}
