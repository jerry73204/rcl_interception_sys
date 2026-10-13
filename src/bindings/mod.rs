//! bindgen output: generated at build time from the ROS headers being built
//! against (see build.rs), or the checked-in file of the `humble` / `jazzy`
//! feature when no ROS is sourced.

#[cfg(rcl_bindings_generated)]
mod distro {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(all(not(rcl_bindings_generated), feature = "humble"))]
#[path = "humble.rs"]
mod distro;

#[cfg(all(
    not(rcl_bindings_generated),
    not(feature = "humble"),
    feature = "jazzy"
))]
#[path = "jazzy.rs"]
mod distro;

pub use distro::*;
