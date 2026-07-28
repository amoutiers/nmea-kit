//! AIS message type definitions.

pub mod aid_to_navigation;
pub mod assignment_mode;
pub mod base_station;
pub mod binary_ack;
pub mod binary_addressed;
pub mod binary_broadcast;
pub mod binary_multi_slot;
pub mod binary_single_slot;
pub mod channel_management;
pub mod common;
pub mod data_link_management;
pub mod dgnss_broadcast;
pub mod group_assignment;
pub mod interrogation;
pub mod long_range;
pub mod position_a;
pub mod position_b;
pub mod position_b_ext;
pub mod safety_addressed;
pub mod safety_broadcast;
pub mod sar_aircraft;
pub mod static_b;
pub mod utc_date_inquiry;
pub mod utc_date_response;
pub(crate) mod utils;
pub mod voyage_a;

pub use aid_to_navigation::AidToNavigation;
pub use assignment_mode::{Assignment, AssignmentModeCommand};
pub use base_station::BaseStationReport;
pub use binary_ack::{AckEntry, BinaryAck};
pub use binary_addressed::BinaryAddressed;
pub use binary_broadcast::BinaryBroadcast;
pub use binary_multi_slot::BinaryMultiSlot;
pub use binary_single_slot::BinarySingleSlot;
pub use channel_management::{ChannelManagement, ChannelManagementScope};
pub use common::*;
pub use data_link_management::{DataLinkManagement, SlotReservation};
pub use dgnss_broadcast::DgnssBroadcast;
pub use group_assignment::GroupAssignment;
pub use interrogation::Interrogation;
pub use long_range::LongRangePosition;
pub use position_a::{ClassBExtendedData, ClassBPositionMetadata, PositionReport};
pub use safety_addressed::SafetyAddressed;
pub use safety_broadcast::SafetyBroadcast;
pub use sar_aircraft::SarAircraftReport;
pub use static_b::StaticDataReport;
pub use utc_date_inquiry::UtcDateInquiry;
pub use utc_date_response::UtcDateResponse;
pub use voyage_a::StaticVoyageData;

#[cfg(test)]
pub(crate) mod test_helpers {
    /// Set `len` bits of `val` (MSB first) at `offset` in a per-bit buffer.
    pub(crate) fn set_bits(buf: &mut [u8], offset: usize, len: usize, val: u32) {
        for i in 0..len {
            buf[offset + i] = ((val >> (len - 1 - i)) & 1) as u8;
        }
    }
}
