//! RMW DDS event types — status structs + enum constants.
//!
//! Used by the interceptor's `rmw_take_event` hook to decode the
//! `event_info` out-parameter after the original returns. Each event
//! type has a distinct status struct; the variant is determined by the
//! `event_type` field of the `rmw_event_t` the caller passes in.
//!
//! Layout matches `/opt/ros/{humble,jazzy}/include/rmw/rmw/events_statuses/*.h`.

#![allow(non_camel_case_types)]

/// `rmw_event_type_t` discriminants — must match the C `enum rmw_event_type_e`
/// from `rmw/event.h`.
pub mod event_type {
    pub const LIVELINESS_CHANGED: i32 = 0;
    pub const REQUESTED_DEADLINE_MISSED: i32 = 1;
    pub const REQUESTED_QOS_INCOMPATIBLE: i32 = 2;
    pub const MESSAGE_LOST: i32 = 3;
    pub const LIVELINESS_LOST: i32 = 4;
    pub const OFFERED_DEADLINE_MISSED: i32 = 5;
    pub const OFFERED_QOS_INCOMPATIBLE: i32 = 6;
    pub const INVALID: i32 = 7;
}

/// `rmw_qos_policy_kind_t` bitmask values from `rmw/qos_policy_kind.h`.
pub mod qos_policy_kind {
    pub const INVALID: i32 = 1 << 0;
    pub const DURABILITY: i32 = 1 << 1;
    pub const DEADLINE: i32 = 1 << 2;
    pub const LIVELINESS: i32 = 1 << 3;
    pub const RELIABILITY: i32 = 1 << 4;
    pub const HISTORY: i32 = 1 << 5;
    pub const LIFESPAN: i32 = 1 << 6;
    pub const DEPTH: i32 = 1 << 7;
    pub const LIVELINESS_LEASE_DURATION: i32 = 1 << 8;
    pub const AVOID_ROS_NAMESPACE_CONVENTIONS: i32 = 1 << 9;
}

/// `rmw_qos_incompatible_event_status_s` — used for both
/// `OFFERED_QOS_INCOMPATIBLE` and `REQUESTED_QOS_INCOMPATIBLE`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RmwQosIncompatibleEventStatus {
    pub total_count: i32,
    pub total_count_change: i32,
    pub last_policy_kind: i32,
}

/// `rmw_requested_deadline_missed_status_s` — also used (identical layout)
/// for `rmw_offered_deadline_missed_status_s`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RmwDeadlineMissedStatus {
    pub total_count: i32,
    pub total_count_change: i32,
}

/// `rmw_liveliness_changed_status_s` — subscription side.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RmwLivelinessChangedStatus {
    pub alive_count: i32,
    pub not_alive_count: i32,
    pub alive_count_change: i32,
    pub not_alive_count_change: i32,
}

/// `rmw_liveliness_lost_status_s` — publisher side.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RmwLivelinessLostStatus {
    pub total_count: i32,
    pub total_count_change: i32,
}

/// `rmw_message_lost_status_s` — note: uses `size_t` (usize) per the
/// header, not i32.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RmwMessageLostStatus {
    pub total_count: usize,
    pub total_count_change: usize,
}

/// The largest status struct (32 bytes — liveliness changed).
/// `rmw_take_event` writes into a buffer of at least this size. Plugins
/// pass a `[u8; 64]` to be safe and cast based on event_type.
pub const MAX_EVENT_STATUS_SIZE: usize = 64;
