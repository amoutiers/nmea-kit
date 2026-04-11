//! AIS message type definitions.

pub mod aid_to_navigation;
pub mod base_station;
pub mod binary_ack;
pub mod binary_addressed;
pub mod binary_broadcast;
pub mod common;
pub mod interrogation;
pub mod long_range;
pub mod position_a;
pub mod position_b;
pub mod position_b_ext;
pub mod safety_addressed;
pub mod safety_broadcast;
pub mod sar_aircraft;
pub mod static_b;
pub mod utc_date_response;
pub(crate) mod utils;
pub mod voyage_a;

pub use aid_to_navigation::AidToNavigation;
pub use base_station::BaseStationReport;
pub use binary_ack::{AckEntry, BinaryAck};
pub use binary_addressed::BinaryAddressed;
pub use binary_broadcast::BinaryBroadcast;
pub use common::*;
pub use interrogation::Interrogation;
pub use long_range::LongRangePosition;
pub use position_a::PositionReport;
pub use safety_addressed::SafetyAddressed;
pub use safety_broadcast::SafetyBroadcast;
pub use sar_aircraft::SarAircraftReport;
pub use static_b::StaticDataReport;
pub use utc_date_response::UtcDateResponse;
pub use voyage_a::StaticVoyageData;
