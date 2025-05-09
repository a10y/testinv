pub mod aux;

use plugin_lib::Plugin;

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
