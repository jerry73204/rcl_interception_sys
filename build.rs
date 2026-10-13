//! Build script for rcl_interception_sys.
//!
//! The struct layouts come from the ROS distribution being built against:
//! bindgen runs on `wrapper.h` with the headers found on `AMENT_PREFIX_PATH`
//! (a sourced ROS) and writes `$OUT_DIR/bindings.rs`. Distributions differ
//! (Jazzy added type hashes; Rolling appended `is_rosidl_buffer_` to
//! `MessageMember`, changing its size), so a new distribution needs no
//! change here.
//!
//! Without ROS headers (a machine with no ROS sourced), the checked-in
//! `src/bindings/<distro>.rs` of the `humble` or `jazzy` feature is used.
//! `generate-bindings` rewrites that checked-in file (developer workflow).

use std::path::{Path, PathBuf};

// Only the ROS packages whose headers we transitively include.
// rcutils: Jazzy's rosidl_runtime_c/type_hash.h includes rcutils/allocator.h.
const REQUIRED_PACKAGES: &[&str] = &[
    "rosidl_runtime_c",
    "rosidl_typesupport_interface",
    "rosidl_typesupport_introspection_c",
    "builtin_interfaces",
    "rcutils",
];

fn main() {
    println!("cargo:rerun-if-env-changed=AMENT_PREFIX_PATH");
    println!("cargo:rerun-if-env-changed=ROS_DISTRO");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo::rustc-check-cfg=cfg(rcl_bindings_generated)");

    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let feature = if cfg!(feature = "humble") {
        Some("humble")
    } else if cfg!(feature = "jazzy") {
        Some("jazzy")
    } else {
        None
    };

    match include_args() {
        Some(args) => {
            let out = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
            generate(&manifest_dir, &args, &out.join("bindings.rs"));
            println!("cargo:rustc-cfg=rcl_bindings_generated");
            if cfg!(feature = "generate-bindings") {
                let distro = std::env::var("ROS_DISTRO")
                    .ok()
                    .or(feature.map(str::to_string))
                    .expect("generate-bindings: set ROS_DISTRO or a distro feature");
                let target = manifest_dir
                    .join("src/bindings")
                    .join(format!("{distro}.rs"));
                std::fs::copy(out.join("bindings.rs"), &target).expect("copy bindings");
                eprintln!("bindgen: wrote {}", target.display());
            }
        }
        None if feature.is_some() => {} // the checked-in bindings of that feature
        None => panic!(
            "rcl_interception_sys: no ROS headers on AMENT_PREFIX_PATH (source a ROS \
             setup.bash), and no `humble`/`jazzy` feature for the checked-in bindings"
        ),
    }
}

/// `-I` flags for the required packages, or None if a ROS is not sourced.
fn include_args() -> Option<Vec<String>> {
    let prefixes: Vec<PathBuf> = std::env::var("AMENT_PREFIX_PATH")
        .ok()?
        .split(':')
        .filter(|p| !p.is_empty())
        .map(PathBuf::from)
        .collect();
    REQUIRED_PACKAGES
        .iter()
        .map(|pkg| {
            prefixes
                .iter()
                .map(|prefix| prefix.join("include").join(pkg))
                .find(|dir| dir.is_dir())
                .map(|dir| format!("-I{}", dir.display()))
        })
        .collect()
}

fn generate(manifest_dir: &Path, include_args: &[String], out_path: &Path) {
    let wrapper_path = manifest_dir.join("wrapper.h");
    let builder = bindgen::Builder::default()
        .header(wrapper_path.to_str().unwrap())
        .clang_args(include_args)
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
    bindings
        .write_to_file(out_path)
        .expect("failed to write bindings");
}
