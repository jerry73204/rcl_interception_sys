//! Pre-generated bindgen output, selected by ROS distro feature.

#[cfg(feature = "humble")]
#[path = "humble.rs"]
mod distro;

#[cfg(feature = "jazzy")]
#[path = "jazzy.rs"]
mod distro;

pub use distro::*;
