//! FFI type definitions mirroring ROS 2 RCL and rosidl C headers.
//!
//! These types are used by LD_PRELOAD interceptor libraries that hook RCL
//! functions. They are `#[repr(C)]` mirrors of the C structs and are only
//! accessed through raw pointers — the interceptor never allocates or
//! constructs them.
//!
//! Source headers (ROS 2 Humble):
//! - `rosidl_runtime_c/message_type_support_struct.h`
//! - `rosidl_typesupport_introspection_c/message_introspection.h`
//! - `rosidl_typesupport_introspection_c/field_types.h`
//! - `builtin_interfaces/msg/detail/time__struct.h`
//! - `rcl/publisher.h`, `rcl/subscription.h`, `rcl/node.h`
//! - `rmw/types.h`

// Type names intentionally match C headers for FFI clarity.
#![allow(non_camel_case_types)]

use std::ffi::{c_char, c_void};

// ---------------------------------------------------------------------------
// rcl_ret_t  (rcl/types.h → rmw/ret_types.h → int32_t)
// ---------------------------------------------------------------------------

/// Return type for rcl functions. 0 = RCL_RET_OK.
pub type rcl_ret_t = i32;

// ---------------------------------------------------------------------------
// rosidl_message_type_support_t  (rosidl_runtime_c/message_type_support_struct.h)
// ---------------------------------------------------------------------------

/// Function pointer type for resolving a specific typesupport handle.
pub type rosidl_message_typesupport_handle_function = Option<
    unsafe extern "C" fn(
        *const rosidl_message_type_support_t,
        *const c_char,
    ) -> *const rosidl_message_type_support_t,
>;

/// `rosidl_message_type_support_t` — carries type-specific support data.
///
/// The `func` pointer resolves to a specific typesupport variant (e.g.
/// introspection_c) when called with the appropriate identifier string.
#[repr(C)]
pub struct rosidl_message_type_support_t {
    pub typesupport_identifier: *const c_char,
    pub data: *const c_void,
    pub func: rosidl_message_typesupport_handle_function,
}

// ---------------------------------------------------------------------------
// MessageMembers / MessageMember  (rosidl_typesupport_introspection_c/message_introspection.h)
// ---------------------------------------------------------------------------

/// Describes a single field of a ROS message (introspection).
///
/// Mirrors `rosidl_typesupport_introspection_c__MessageMember`.
/// Field order and types match the C header exactly.
#[repr(C)]
pub struct MessageMember {
    /// Field name (null-terminated C string).
    pub name_: *const c_char,
    /// Field type as a `rosidl_typesupport_introspection_c_field_types` value.
    pub type_id_: u8,
    /// Upper bound on string length (0 if not a string).
    pub string_upper_bound_: usize,
    /// If `type_id_ == ROS_TYPE_MESSAGE`, points to sub-message typesupport.
    pub members_: *const rosidl_message_type_support_t,
    /// True if this field is an array.
    pub is_array_: bool,
    /// Number of elements if fixed-size array, or 0 for dynamic.
    pub array_size_: usize,
    /// If true, `array_size_` is an upper bound (bounded sequence).
    pub is_upper_bound_: bool,
    /// Byte offset of this field within the message struct.
    pub offset_: u32,
    /// Pointer to default value, or null.
    pub default_value_: *const c_void,
    // --- Array access function pointers (6 total) ---
    pub size_function: Option<unsafe extern "C" fn(*const c_void) -> usize>,
    pub get_const_function: Option<unsafe extern "C" fn(*const c_void, usize) -> *const c_void>,
    pub get_function: Option<unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void>,
    pub fetch_function: Option<unsafe extern "C" fn(*const c_void, usize, *mut c_void)>,
    pub assign_function: Option<unsafe extern "C" fn(*mut c_void, usize, *const c_void)>,
    pub resize_function: Option<unsafe extern "C" fn(*mut c_void, usize) -> bool>,
}

/// Describes all fields of a ROS message type (introspection).
///
/// Mirrors `rosidl_typesupport_introspection_c__MessageMembers`.
#[repr(C)]
pub struct MessageMembers {
    /// Package + subfolder namespace, e.g. `"sensor_msgs__msg"`.
    pub message_namespace_: *const c_char,
    /// Message name, e.g. `"PointCloud2"`.
    pub message_name_: *const c_char,
    /// Number of fields.
    pub member_count_: u32,
    /// `sizeof()` of the message C struct.
    pub size_of_: usize,
    /// Pointer to array of `member_count_` `MessageMember` entries.
    pub members_: *const MessageMember,
    /// Initialization function.
    pub init_function: Option<unsafe extern "C" fn(*mut c_void, i32)>,
    /// Finalization function.
    pub fini_function: Option<unsafe extern "C" fn(*mut c_void)>,
}

// ---------------------------------------------------------------------------
// Field type constants  (rosidl_typesupport_introspection_c/field_types.h)
// ---------------------------------------------------------------------------

/// Field type value indicating an embedded message (sub-struct).
pub const ROS_TYPE_MESSAGE: u8 = 18;

// ---------------------------------------------------------------------------
// builtin_interfaces/msg/Time  (builtin_interfaces/msg/detail/time__struct.h)
// ---------------------------------------------------------------------------

/// ROS 2 `builtin_interfaces/msg/Time` — the stamp type inside `std_msgs/Header`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuiltinTime {
    pub sec: i32,
    pub nanosec: u32,
}

// ---------------------------------------------------------------------------
// Opaque RCL handle types
//
// These are used only as pointer targets in function signatures. The
// interceptor never dereferences them — it passes them through to the
// original functions. Using distinct types prevents accidental mixing
// of publisher/subscription/node pointers.
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Typesupport identifier constant
// ---------------------------------------------------------------------------

/// The identifier string used to request the introspection_c typesupport.
pub const TYPESUPPORT_INTROSPECTION_C_IDENTIFIER: &[u8] = b"rosidl_typesupport_introspection_c\0";

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::mem::offset_of;

    use super::*;

    // All expected values below come from compiling check_layout.c with gcc
    // against the installed ROS 2 Humble headers on x86_64 Linux.

    #[test]
    fn builtin_time_layout() {
        assert_eq!(size_of::<BuiltinTime>(), 8);
        assert_eq!(align_of::<BuiltinTime>(), 4);
        assert_eq!(offset_of!(BuiltinTime, sec), 0);
        assert_eq!(offset_of!(BuiltinTime, nanosec), 4);
    }

    #[test]
    fn type_support_layout() {
        assert_eq!(size_of::<rosidl_message_type_support_t>(), 24);
        assert_eq!(align_of::<rosidl_message_type_support_t>(), 8);
        assert_eq!(
            offset_of!(rosidl_message_type_support_t, typesupport_identifier),
            0
        );
        assert_eq!(offset_of!(rosidl_message_type_support_t, data), 8);
        assert_eq!(offset_of!(rosidl_message_type_support_t, func), 16);
    }

    #[test]
    fn message_members_layout() {
        assert_eq!(size_of::<MessageMembers>(), 56);
        assert_eq!(align_of::<MessageMembers>(), 8);
        assert_eq!(offset_of!(MessageMembers, message_namespace_), 0);
        assert_eq!(offset_of!(MessageMembers, message_name_), 8);
        assert_eq!(offset_of!(MessageMembers, member_count_), 16);
        assert_eq!(offset_of!(MessageMembers, size_of_), 24);
        assert_eq!(offset_of!(MessageMembers, members_), 32);
        assert_eq!(offset_of!(MessageMembers, init_function), 40);
        assert_eq!(offset_of!(MessageMembers, fini_function), 48);
    }

    #[test]
    fn message_member_layout() {
        assert_eq!(size_of::<MessageMember>(), 112);
        assert_eq!(align_of::<MessageMember>(), 8);
        assert_eq!(offset_of!(MessageMember, name_), 0);
        assert_eq!(offset_of!(MessageMember, type_id_), 8);
        assert_eq!(offset_of!(MessageMember, string_upper_bound_), 16);
        assert_eq!(offset_of!(MessageMember, members_), 24);
        assert_eq!(offset_of!(MessageMember, is_array_), 32);
        assert_eq!(offset_of!(MessageMember, array_size_), 40);
        assert_eq!(offset_of!(MessageMember, is_upper_bound_), 48);
        assert_eq!(offset_of!(MessageMember, offset_), 52);
        assert_eq!(offset_of!(MessageMember, default_value_), 56);
        assert_eq!(offset_of!(MessageMember, size_function), 64);
        assert_eq!(offset_of!(MessageMember, get_const_function), 72);
        assert_eq!(offset_of!(MessageMember, get_function), 80);
        assert_eq!(offset_of!(MessageMember, fetch_function), 88);
        assert_eq!(offset_of!(MessageMember, assign_function), 96);
        assert_eq!(offset_of!(MessageMember, resize_function), 104);
    }

    #[test]
    fn ros_type_message_value() {
        assert_eq!(ROS_TYPE_MESSAGE, 18);
    }

    #[test]
    fn typesupport_identifier_null_terminated() {
        assert_eq!(*TYPESUPPORT_INTROSPECTION_C_IDENTIFIER.last().unwrap(), 0u8);
    }
}
