use crate::gender::Gender;
use crate::Codename;
use super::EventCompareType;
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
    Track500m,

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
    Track2Miles,
    Track2000m,
    Track3000m,
    Track5000m,
    Track10000m,

    // Long-distance events (non-track)
    Road5km,
    Road10km,
    Road15km,
    Road20km,
    Road25km,
    Road30km,
    Road100km,
    Road10Miles,
    HalfMarathon,
    Marathon,

    // NOTE: in the iaaf_2017 tables, there is no difference between road or track walking!
    // Walking events (road)
    Walk3km,
    Walk5km,
    Walk10km,
    Walk15km,
    Walk20km,
    Walk30km,
    Walk35km,
    Walk50km,

    // Walking events (track)
    Walk3kmTrack,
    Walk5kmTrack,
    Walk10kmTrack,
    Walk15kmTrack,
    Walk20kmTrack,
    Walk30kmTrack,
    Walk35kmTrack,
    Walk50kmTrack,
}

impl Codename for OutdoorEvent {
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
            OutdoorEvent::Track500m => "500m",
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
            OutdoorEvent::Track3000m => "3000m",
            OutdoorEvent::Track1Mile => "Mile",
            OutdoorEvent::Track2Miles => "2 Miles",
            OutdoorEvent::Track2000m => "2000m",
            OutdoorEvent::Track5000m => "5000m",
            OutdoorEvent::Track10000m => "10000m",
            OutdoorEvent::Road5km => "5 km",
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
            OutdoorEvent::Walk15km => "15km W",
            OutdoorEvent::Walk20km => "20km W",
            OutdoorEvent::Walk30km => "30km W",
            OutdoorEvent::Walk50km => "50km W",
            OutdoorEvent::Walk35km => "35km W",
            OutdoorEvent::Walk3kmTrack => "3000mW",
            OutdoorEvent::Walk5kmTrack => "5000mW",
            OutdoorEvent::Walk10kmTrack => "10000mW",
            OutdoorEvent::Walk15kmTrack => "15000mW",
            OutdoorEvent::Walk20kmTrack => "20000mW",
            OutdoorEvent::Walk30kmTrack => "30000mW",
            OutdoorEvent::Walk35kmTrack => "35000mW",
            OutdoorEvent::Walk50kmTrack => "50000mW",
        }
    }

    fn from_codename(code: &str) -> Option<Self> {
        let event = match code {
            "HJ" => OutdoorEvent::HighJump,
            "LJ" => OutdoorEvent::LongJump,
            "TJ" => OutdoorEvent::TripleJump,
            "DT" => OutdoorEvent::DiscusThrow,
            "HT" => OutdoorEvent::HammerThrow,
            "JT" => OutdoorEvent::JavelinThrow,
            "PV" => OutdoorEvent::PoleVault,
            "SP" => OutdoorEvent::ShortPut,
            "Heptathlon" => OutdoorEvent::Heptathlon,
            "Decathlon" => OutdoorEvent::Decathlon,
            "100m" => OutdoorEvent::Track100m,
            "200m" => OutdoorEvent::Track200m,
            "300m" => OutdoorEvent::Track300m,
            "400m" => OutdoorEvent::Track400m,
            "500m" => OutdoorEvent::Track500m,
            "100mH" => OutdoorEvent::Track100mHurdles,
            "110mH" => OutdoorEvent::Track110mHurdles,
            "400mH" => OutdoorEvent::Track400mHurdles,
            "2000m SC" => OutdoorEvent::Track2kmSteeplechase,
            "3000m SC" => OutdoorEvent::Track3kmSteeplechase,
            "4x100m" => OutdoorEvent::Track4x100mRelay,
            "4x200m" => OutdoorEvent::Track4x200mRelay,
            "4x400m" => OutdoorEvent::Track4x400mRelay,
            "600m" => OutdoorEvent::Track600m,
            "800m" => OutdoorEvent::Track800m,
            "1000m" => OutdoorEvent::Track1000m,
            "1500m" => OutdoorEvent::Track1500m,
            "3000m" => OutdoorEvent::Track3000m,
            "Mile" => OutdoorEvent::Track1Mile,
            "2 Miles" => OutdoorEvent::Track2Miles,
            "2000m" => OutdoorEvent::Track2000m,
            "5000m" => OutdoorEvent::Track5000m,
            "10000m" => OutdoorEvent::Track10000m,
            "5 km" => OutdoorEvent::Road5km,
            "10 km" => OutdoorEvent::Road10km,
            "15 km" => OutdoorEvent::Road15km,
            "20 km" => OutdoorEvent::Road20km,
            "25 km" => OutdoorEvent::Road25km,
            "30 km" => OutdoorEvent::Road30km,
            "100 km" => OutdoorEvent::Road100km,
            "10 Miles" => OutdoorEvent::Road10Miles,
            "HM" => OutdoorEvent::HalfMarathon,
            "Marathon" => OutdoorEvent::Marathon,
            "3km W" => OutdoorEvent::Walk3km,
            "5km W" => OutdoorEvent::Walk5km,
            "10km W" => OutdoorEvent::Walk10km,
            "15km W" => OutdoorEvent::Walk15km,
            "20km W" => OutdoorEvent::Walk20km,
            "30km W" => OutdoorEvent::Walk30km,
            "50km W" => OutdoorEvent::Walk50km,
            "35km W" => OutdoorEvent::Walk35km,
            "3000mW" => OutdoorEvent::Walk3kmTrack,
            "5000mW" => OutdoorEvent::Walk5kmTrack,
            "10000mW" => OutdoorEvent::Walk10kmTrack,
            "15000mW" => OutdoorEvent::Walk15kmTrack,
            "20000mW" => OutdoorEvent::Walk20kmTrack,
            "30000mW" => OutdoorEvent::Walk30kmTrack,
            "35000mW" => OutdoorEvent::Walk35kmTrack,
            "50000mW" => OutdoorEvent::Walk50kmTrack,
            _ => return None,
        };

        Some(event)
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
            OutdoorEvent::Walk35km => Some(Gender::Male),
            _ => None,
        }
    }

    pub fn exists_for_gender(&self, gender: &Gender) -> bool {
        match self.exclusive_for_gender() {
            None => true,
            Some(specific_gender) => &specific_gender == gender,
        }
    }

    pub fn compare_type(&self) -> EventCompareType {
        match self {
            OutdoorEvent::HighJump => EventCompareType::GreaterIsBetter,
            OutdoorEvent::LongJump => EventCompareType::GreaterIsBetter,
            OutdoorEvent::TripleJump => EventCompareType::GreaterIsBetter,
            OutdoorEvent::DiscusThrow => EventCompareType::GreaterIsBetter,
            OutdoorEvent::HammerThrow => EventCompareType::GreaterIsBetter,
            OutdoorEvent::JavelinThrow => EventCompareType::GreaterIsBetter,
            OutdoorEvent::PoleVault => EventCompareType::GreaterIsBetter,
            OutdoorEvent::ShortPut => EventCompareType::GreaterIsBetter,
            OutdoorEvent::Heptathlon => EventCompareType::GreaterIsBetter,
            OutdoorEvent::Decathlon => EventCompareType::GreaterIsBetter,

            OutdoorEvent::Track100m => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track200m => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track300m => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track400m => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track500m => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track100mHurdles => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track110mHurdles => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track400mHurdles => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track2kmSteeplechase => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track3kmSteeplechase => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track4x100mRelay => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track4x200mRelay => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track4x400mRelay => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track600m => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track800m => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track1000m => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track1500m => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track3000m => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track1Mile => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track2Miles => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track2000m => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track5000m => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Track10000m => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Road5km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Road10km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Road15km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Road20km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Road25km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Road30km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Road100km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Road10Miles => EventCompareType::SmallerIsBetter,
            OutdoorEvent::HalfMarathon => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Marathon => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk3km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk5km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk10km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk15km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk20km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk30km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk50km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk35km => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk3kmTrack => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk5kmTrack => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk10kmTrack => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk15kmTrack => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk20kmTrack => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk30kmTrack => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk35kmTrack => EventCompareType::SmallerIsBetter,
            OutdoorEvent::Walk50kmTrack => EventCompareType::SmallerIsBetter,
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
