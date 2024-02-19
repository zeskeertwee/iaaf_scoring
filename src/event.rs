use crate::gender::Gender;
use crate::ToCodename;
use strum::EnumIter;

#[derive(Clone, Copy, Eq, PartialEq, Hash, EnumIter, Debug)]
pub enum OutdoorEvent {
    // Non-running events
    HighJump,
    LongJump,
    TripleJump,
    DiscusThrow,
    HammerThrow,
    JavelinThrow,
    PoleVault,
    ShortPut,

    /// This event only has scoring tables for female contestants
    Heptathlon,
    /// This event only has scoring tables for male contestants
    Decathlon,

    // Sprint events
    Track100m,
    Track200m,
    Track300m,
    Track400m,

    // Sprint events with hurdles
    /// This event only has scoring tables for female contestants
    Track100mHurdles,
    /// This event only has scoring tables for male contestants
    Track110mHurdles,
    Track400mHurdles,
    Track2kmSteeplechase,
    Track3kmSteeplechase,

    // Relay sprint events
    Track4x100mRelay,
    Track4x200mRelay,
    Track4x400mRelay,

    // Medium-distance events
    Track600m,
    Track800m,
    Track1000m,
    Track1500m,
    Track1Mile,
    Track2000m,
    Track5000m,
    Track10000m,

    // Long-distance events (non-track)
    Road10km,
    Road15km,
    Road20km,
    Road25km,
    Road30km,
    Road100km,
    Road10Miles,
    HalfMarathon,
    Marathon,

    // Walking events
    Walk3km,
    Walk5km,
    Walk10km,
    Walk20km,
    Walk30km,
    Walk50km,
}

impl ToCodename for OutdoorEvent {
    fn to_codename(&self) -> &str {
        match self {
            OutdoorEvent::HighJump => "HJ",
            OutdoorEvent::LongJump => "LJ",
            OutdoorEvent::TripleJump => "TJ",
            OutdoorEvent::DiscusThrow => "DT",
            OutdoorEvent::HammerThrow => "HT",
            OutdoorEvent::JavelinThrow => "JT",
            OutdoorEvent::PoleVault => "PV",
            OutdoorEvent::ShortPut => "SP",
            OutdoorEvent::Heptathlon => "Heptathlon",
            OutdoorEvent::Decathlon => "Decathlon",
            OutdoorEvent::Track100m => "100m",
            OutdoorEvent::Track200m => "200m",
            OutdoorEvent::Track300m => "300m",
            OutdoorEvent::Track400m => "400m",
            OutdoorEvent::Track100mHurdles => "100mH",
            OutdoorEvent::Track110mHurdles => "110mH",
            OutdoorEvent::Track400mHurdles => "400mH",
            OutdoorEvent::Track2kmSteeplechase => "2000m SC",
            OutdoorEvent::Track3kmSteeplechase => "3000m SC",
            OutdoorEvent::Track4x100mRelay => "4x100m",
            OutdoorEvent::Track4x200mRelay => "4x200m",
            OutdoorEvent::Track4x400mRelay => "4x400m",
            OutdoorEvent::Track600m => "600m",
            OutdoorEvent::Track800m => "800m",
            OutdoorEvent::Track1000m => "1000m",
            OutdoorEvent::Track1500m => "1500m",
            OutdoorEvent::Track1Mile => "Mile",
            OutdoorEvent::Track2000m => "2000m",
            OutdoorEvent::Track5000m => "5000m",
            OutdoorEvent::Track10000m => "10000m",
            OutdoorEvent::Road10km => "10 km",
            OutdoorEvent::Road15km => "15 km",
            OutdoorEvent::Road20km => "20 km",
            OutdoorEvent::Road25km => "25 km",
            OutdoorEvent::Road30km => "30 km",
            OutdoorEvent::Road100km => "100 km",
            OutdoorEvent::Road10Miles => "10 Miles",
            OutdoorEvent::HalfMarathon => "HM",
            OutdoorEvent::Marathon => "Marathon",
            OutdoorEvent::Walk3km => "3km W",
            OutdoorEvent::Walk5km => "5km W",
            OutdoorEvent::Walk10km => "10km W",
            OutdoorEvent::Walk20km => "20km W",
            OutdoorEvent::Walk30km => "30km W",
            OutdoorEvent::Walk50km => "50km W",
        }
    }
}

impl OutdoorEvent {
    pub fn is_gender_specific(&self) -> bool {
        self.exclusive_for_gender().is_some()
    }

    pub fn exclusive_for_gender(&self) -> Option<Gender> {
        match self {
            OutdoorEvent::Track110mHurdles => Some(Gender::Male),
            OutdoorEvent::Track100mHurdles => Some(Gender::Female),
            OutdoorEvent::Heptathlon => Some(Gender::Female),
            OutdoorEvent::Decathlon => Some(Gender::Male),
            _ => None,
        }
    }

    pub fn exists_for_gender(&self, gender: &Gender) -> bool {
        match self.exclusive_for_gender() {
            None => true,
            Some(specific_gender) => &specific_gender == gender,
        }
    }
}

#[test]
fn test_gender_specific_events() {
    assert!(OutdoorEvent::Track110mHurdles.exists_for_gender(&Gender::Male));
    assert!(!OutdoorEvent::Track110mHurdles.exists_for_gender(&Gender::Female));

    assert!(!OutdoorEvent::HighJump.is_gender_specific());

    assert!(OutdoorEvent::LongJump.exists_for_gender(&Gender::Female));
    assert!(OutdoorEvent::LongJump.exists_for_gender(&Gender::Male));
}
