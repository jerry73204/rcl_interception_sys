//! Layout-aware view of `rmw_qos_profile_t` (Phase 36.2).
//!
//! The opaque type in `opaque::rmw_qos_profile_t` is sufficient when the
//! interceptor only forwards the pointer. To inspect QoS policy values
//! we cast the opaque pointer to [`RmwQosProfile`] which matches the
//! C struct layout from `rmw/types.h`.
//!
//! The layout is stable across Humble and Jazzy (verified against
//! `/opt/ros/{humble,jazzy}/include/rmw/rmw/types.h`).
//!
//! All enum values match the C `enum rmw_qos_*_policy_e` discriminants
//! (sequential from 0). The liveliness enum has a 1-value gap:
//! AUTOMATIC=1, MANUAL_BY_TOPIC=3.

#![allow(non_camel_case_types)]

/// Duration (`rmw_time_s`). 16 bytes.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RmwTime {
    pub sec: u64,
    pub nsec: u64,
}

/// Layout-aware `rmw_qos_profile_t`. Fields ordered to match the C
/// struct exactly on Linux x86_64 (and aarch64, which uses the same
/// natural alignment rules).
///
/// Padding bytes are explicit `_pad*` fields so the Rust struct lays
/// out identically to the C type.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RmwQosProfile {
    /// `enum rmw_qos_history_policy_e` — i32 (SYSTEM_DEFAULT=0,
    /// KEEP_LAST=1, KEEP_ALL=2, UNKNOWN=3).
    pub history: i32,
    /// Padding so `depth` lands on its 8-byte alignment.
    pub _pad0: u32,
    /// `size_t` — message queue depth.
    pub depth: usize,
    /// `enum rmw_qos_reliability_policy_e` — i32 (SYSTEM_DEFAULT=0,
    /// RELIABLE=1, BEST_EFFORT=2, UNKNOWN=3).
    pub reliability: i32,
    /// `enum rmw_qos_durability_policy_e` — i32 (SYSTEM_DEFAULT=0,
    /// TRANSIENT_LOCAL=1, VOLATILE=2, UNKNOWN=3).
    pub durability: i32,
    /// Period at which messages must be sent/received.
    pub deadline: RmwTime,
    /// Age at which messages expire.
    pub lifespan: RmwTime,
    /// `enum rmw_qos_liveliness_policy_e` — i32 (SYSTEM_DEFAULT=0,
    /// AUTOMATIC=1, MANUAL_BY_TOPIC=3). Note: 2 is skipped.
    pub liveliness: i32,
    /// Padding so `liveliness_lease_duration` lands on 8-byte alignment.
    pub _pad1: u32,
    pub liveliness_lease_duration: RmwTime,
    /// If true, circumvent ROS namespace conventions (advanced use).
    pub avoid_ros_namespace_conventions: bool,
    /// Trailing padding to match the C struct's 8-byte tail alignment.
    pub _pad2: [u8; 7],
}

// Compile-time check: the struct should be at least 88 bytes (matches
// the C layout). If a distro reorders fields this assertion fires.
const _: () = assert!(core::mem::size_of::<RmwQosProfile>() >= 88);

// Explicit offset checks for the fields the runtime reads. These match
// the C `rmw_qos_profile_s` layout on Linux x86_64 (Humble + Jazzy).
const _: () = {
    assert!(core::mem::offset_of!(RmwQosProfile, history) == 0);
    assert!(core::mem::offset_of!(RmwQosProfile, depth) == 8);
    assert!(core::mem::offset_of!(RmwQosProfile, reliability) == 16);
    assert!(core::mem::offset_of!(RmwQosProfile, durability) == 20);
    assert!(core::mem::offset_of!(RmwQosProfile, deadline) == 24);
    assert!(core::mem::offset_of!(RmwQosProfile, lifespan) == 40);
    assert!(core::mem::offset_of!(RmwQosProfile, liveliness) == 56);
    assert!(core::mem::offset_of!(RmwQosProfile, liveliness_lease_duration) == 64);
    assert!(core::mem::offset_of!(RmwQosProfile, avoid_ros_namespace_conventions) == 80);
};

// ---------------------------------------------------------------------------
// Enum value constants — i32 discriminants matching the C enums
// ---------------------------------------------------------------------------

pub mod reliability {
    pub const SYSTEM_DEFAULT: i32 = 0;
    pub const RELIABLE: i32 = 1;
    pub const BEST_EFFORT: i32 = 2;
    pub const UNKNOWN: i32 = 3;
}

pub mod durability {
    pub const SYSTEM_DEFAULT: i32 = 0;
    pub const TRANSIENT_LOCAL: i32 = 1;
    pub const VOLATILE: i32 = 2;
    pub const UNKNOWN: i32 = 3;
}

pub mod history {
    pub const SYSTEM_DEFAULT: i32 = 0;
    pub const KEEP_LAST: i32 = 1;
    pub const KEEP_ALL: i32 = 2;
    pub const UNKNOWN: i32 = 3;
}

pub mod liveliness {
    pub const SYSTEM_DEFAULT: i32 = 0;
    pub const AUTOMATIC: i32 = 1;
    pub const MANUAL_BY_TOPIC: i32 = 3;
    pub const UNKNOWN: i32 = 4;
}
