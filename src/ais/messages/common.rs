//! Common AIS types: NavigationStatus, ShipType.

/// AIS Navigation Status (4 bits, 0-15).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationStatus {
    UnderWayEngine,
    AtAnchor,
    NotUnderCommand,
    RestrictedManeuverability,
    ConstrainedByDraught,
    Moored,
    Aground,
    EngagedInFishing,
    UnderWaySailing,
    ReservedHsc,
    ReservedWig,
    Reserved(u8),
    AisSartActive,
    Undefined,
}

impl From<u8> for NavigationStatus {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::UnderWayEngine,
            1 => Self::AtAnchor,
            2 => Self::NotUnderCommand,
            3 => Self::RestrictedManeuverability,
            4 => Self::ConstrainedByDraught,
            5 => Self::Moored,
            6 => Self::Aground,
            7 => Self::EngagedInFishing,
            8 => Self::UnderWaySailing,
            9 => Self::ReservedHsc,
            10 => Self::ReservedWig,
            14 => Self::AisSartActive,
            15 => Self::Undefined,
            other => Self::Reserved(other),
        }
    }
}

impl From<NavigationStatus> for u8 {
    fn from(val: NavigationStatus) -> Self {
        match val {
            NavigationStatus::UnderWayEngine => 0,
            NavigationStatus::AtAnchor => 1,
            NavigationStatus::NotUnderCommand => 2,
            NavigationStatus::RestrictedManeuverability => 3,
            NavigationStatus::ConstrainedByDraught => 4,
            NavigationStatus::Moored => 5,
            NavigationStatus::Aground => 6,
            NavigationStatus::EngagedInFishing => 7,
            NavigationStatus::UnderWaySailing => 8,
            NavigationStatus::ReservedHsc => 9,
            NavigationStatus::ReservedWig => 10,
            NavigationStatus::AisSartActive => 14,
            NavigationStatus::Undefined => 15,
            NavigationStatus::Reserved(v) => v,
        }
    }
}

/// AIS transceiver class, inferred from message type.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AisClass {
    /// Class A — SOLAS vessels (Types 1/2/3/5)
    A,
    /// Class B "SO" — leisure/small craft (Type 18/24)
    B,
    /// Class B+ "CS" — enhanced Class B (Type 19)
    BPlus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_status_known_values() {
        assert_eq!(NavigationStatus::from(0), NavigationStatus::UnderWayEngine);
        assert_eq!(NavigationStatus::from(1), NavigationStatus::AtAnchor);
        assert_eq!(NavigationStatus::from(5), NavigationStatus::Moored);
        assert_eq!(NavigationStatus::from(8), NavigationStatus::UnderWaySailing);
        assert_eq!(u8::from(NavigationStatus::Moored), 5);
    }

    #[test]
    fn nav_status_roundtrip() {
        for val in 0..=15u8 {
            let status = NavigationStatus::from(val);
            let back: u8 = status.into();
            assert_eq!(val, back);
        }
    }
}
