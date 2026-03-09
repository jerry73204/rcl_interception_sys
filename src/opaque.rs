//! Opaque RCL handle types.
//!
//! These are used only as pointer targets in function signatures. The
//! interceptor never dereferences them — it passes them through to the
//! original functions. Using distinct types prevents accidental mixing
//! of publisher/subscription/node pointers.
//!
//! Blocklisted from bindgen to avoid pulling in the massive
//! `rmw_qos_profile_t` dependency tree.

// Type names intentionally match C headers for FFI clarity.
#![allow(non_camel_case_types)]

/// Opaque `rcl_node_t`.
#[repr(C)]
pub struct rcl_node_t {
    _opaque: [u8; 0],
}

/// Opaque `rcl_publisher_t`.
#[repr(C)]
pub struct rcl_publisher_t {
    _opaque: [u8; 0],
}

/// Opaque `rcl_subscription_t`.
#[repr(C)]
pub struct rcl_subscription_t {
    _opaque: [u8; 0],
}

/// Opaque `rcl_publisher_options_t`.
#[repr(C)]
pub struct rcl_publisher_options_t {
    _opaque: [u8; 0],
}

/// Opaque `rcl_subscription_options_t`.
#[repr(C)]
pub struct rcl_subscription_options_t {
    _opaque: [u8; 0],
}

/// Opaque `rmw_publisher_allocation_t`.
#[repr(C)]
pub struct rmw_publisher_allocation_t {
    _opaque: [u8; 0],
}

/// Opaque `rmw_subscription_allocation_t`.
#[repr(C)]
pub struct rmw_subscription_allocation_t {
    _opaque: [u8; 0],
}

/// Opaque `rmw_message_info_t`.
#[repr(C)]
pub struct rmw_message_info_t {
    _opaque: [u8; 0],
}
