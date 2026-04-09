//! Shared AIS decode helpers used across multiple message types.

/// Decode latitude from 1/10000 minute to degrees. 91° = not available.
pub(crate) fn decode_latitude(raw: i32) -> Option<f64> {
    let deg = f64::from(raw) / 600_000.0;
    if !(-90.0..=90.0).contains(&deg) {
        None
    } else {
        Some(deg)
    }
}

/// Decode longitude from 1/10000 minute to degrees. 181° = not available.
pub(crate) fn decode_longitude(raw: i32) -> Option<f64> {
    let deg = f64::from(raw) / 600_000.0;
    if !(-180.0..=180.0).contains(&deg) {
        None
    } else {
        Some(deg)
    }
}

/// Decode SOG from 1/10 knot. 1023 = not available.
pub(crate) fn decode_sog(raw: u32) -> Option<f32> {
    if raw == 1023 {
        None
    } else {
        Some(raw as f32 / 10.0)
    }
}

/// Decode COG from 1/10 degree. 3600 = not available.
pub(crate) fn decode_cog(raw: u32) -> Option<f32> {
    if raw == 3600 {
        None
    } else {
        Some(raw as f32 / 10.0)
    }
}

/// Decode true heading in degrees. 511 = not available.
pub(crate) fn decode_heading(raw: u32) -> Option<u16> {
    if raw == 511 { None } else { Some(raw as u16) }
}

/// Decode rate of turn. -128 = not available.
pub(crate) fn decode_rot(raw: i32) -> Option<f32> {
    if raw == -128 { None } else { Some(raw as f32) }
}
