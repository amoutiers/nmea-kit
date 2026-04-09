//! AIS message type definitions.

pub mod aid_to_navigation;
pub mod base_station;
pub mod common;
pub mod long_range;
pub mod position_a;
pub mod position_b;
pub mod position_b_ext;
pub mod safety_broadcast;
pub mod static_b;
pub(crate) mod utils;
pub mod voyage_a;

pub use aid_to_navigation::AidToNavigation;
pub use base_station::BaseStationReport;
pub use common::*;
pub use long_range::LongRangePosition;
pub use position_a::PositionReport;
pub use safety_broadcast::SafetyBroadcast;
pub use static_b::StaticDataReport;
pub use voyage_a::StaticVoyageData;
