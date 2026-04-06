//! AIS message type definitions.

pub mod common;
pub mod position_a;
pub mod position_b;
pub mod position_b_ext;
pub mod static_b;
pub mod voyage_a;

pub use common::*;
pub use position_a::PositionReport;
pub use static_b::StaticDataReport;
pub use voyage_a::StaticVoyageData;
