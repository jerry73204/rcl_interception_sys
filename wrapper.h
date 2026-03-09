// Bindgen input — includes the ROS 2 C headers we need FFI types for.
//
// These headers define the structs used by the LD_PRELOAD interceptor:
//   - rosidl_message_type_support_t  (type support dispatch)
//   - MessageMembers / MessageMember (introspection for stamp offset)
//   - builtin_interfaces/msg/Time    (the stamp type)
//   - field_types constants          (ROS_TYPE_MESSAGE = 18)

#include <rosidl_runtime_c/message_type_support_struct.h>
#include <rosidl_typesupport_introspection_c/message_introspection.h>
#include <rosidl_typesupport_introspection_c/field_types.h>
#include <builtin_interfaces/msg/detail/time__struct.h>
