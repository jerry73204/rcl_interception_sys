//! Build script for rcl_interception_sys.
//!
//! Without `generate-bindings`: no-op (pre-generated bindings used directly).
//! With `generate-bindings`: runs bindgen on wrapper.h to produce src/bindings/<distro>.rs.

fn main() {
    #[cfg(feature = "generate-bindings")]
    generate();
}

#[cfg(feature = "generate-bindings")]
fn generate() {
    use std::path::PathBuf;

    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));

    let distro = if cfg!(feature = "humble") {
        "humble"
    } else if cfg!(feature = "jazzy") {
        "jazzy"
    } else {
        panic!("generate-bindings requires exactly one of: humble, jazzy");
    };

    // Only the ROS packages whose headers we transitively include.
    const REQUIRED_PACKAGES: &[&str] = &[
        "rosidl_runtime_c",
        "rosidl_typesupport_interface",
        "rosidl_typesupport_introspection_c",
        "builtin_interfaces",
    ];

    // Search all AMENT_PREFIX_PATH entries for the required package include dirs.
    let prefixes: Vec<PathBuf> = std::env::var("AMENT_PREFIX_PATH")
        .ok()
        .map(|p| p.split(':').map(PathBuf::from).collect())
        .unwrap_or_else(|| vec![PathBuf::from(format!("/opt/ros/{distro}"))]);

    let mut include_args: Vec<String> = Vec::new();
    for pkg in REQUIRED_PACKAGES {
        let mut found = false;
        for prefix in &prefixes {
            let pkg_dir = prefix.join("include").join(pkg);
            if pkg_dir.is_dir() {
                include_args.push(format!("-I{}", pkg_dir.display()));
                found = true;
                break;
            }
        }
        assert!(found, "ROS package include dir not found: {pkg}");
    }

    let wrapper_path = manifest_dir.join("wrapper.h");
    assert!(
        wrapper_path.exists(),
        "wrapper.h not found at {}",
        wrapper_path.display()
    );

    let builder = bindgen::Builder::default()
        .header(wrapper_path.to_str().unwrap())
        .clang_args(&include_args)
        // Types we want generated
        .allowlist_type("rosidl_message_type_support_t")
        .allowlist_type("rosidl_typesupport_introspection_c__MessageMembers")
        .allowlist_type("rosidl_typesupport_introspection_c__MessageMember")
        .allowlist_type("builtin_interfaces__msg__Time")
        // Field type enum (generates constants like ROS_TYPE_MESSAGE = 18)
        .allowlist_type("rosidl_typesupport_introspection_c_field_types")
        // Blocklist opaque types we handle by hand
        .blocklist_type("rcl_node_t")
        .blocklist_type("rcl_publisher_t")
        .blocklist_type("rcl_subscription_t")
        .blocklist_type("rcl_publisher_options_t")
        .blocklist_type("rcl_subscription_options_t")
        .blocklist_type("rmw_publisher_allocation_t")
        .blocklist_type("rmw_subscription_allocation_t")
        .blocklist_type("rmw_message_info_t")
        // No functions — we use dlsym, not linking
        .allowlist_function("^$")
        // Generate layout tests
        .layout_tests(true)
        .derive_debug(true)
        .derive_default(false)
        .derive_copy(true)
        .merge_extern_blocks(true)
        .generate_comments(true);

    let bindings = builder.generate().expect("bindgen failed");

    let out_path = manifest_dir
        .join("src/bindings")
        .join(format!("{distro}.rs"));
    bindings
        .write_to_file(&out_path)
        .expect("failed to write bindings");

    eprintln!("bindgen: wrote {}", out_path.display());
}
