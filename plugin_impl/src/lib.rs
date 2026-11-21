//! Plugin implementations that register with the `plugin_lib` registry.

pub mod aux;

use plugin_lib::Plugin;

/// Forces this crate to be linked into the final binary.
///
/// The binary references this so the `inventory::submit!` registrations survive linking;
/// see [`aux::force_link`].
pub fn force_link() {}

inventory::submit! {
    Plugin { message: "pluginA" } 
}
inventory::submit! {
    Plugin { message: "pluginB" } 
}
inventory::submit! {
    Plugin { message: "pluginC" } 
}
