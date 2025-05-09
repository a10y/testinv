use plugin_lib::*;

// this will get the plugin impl included, but ONLY if i import a function
// from the same module module as the registry is in.
#[used]
static PLUGIN_IMPORT: fn() = || {
    plugin_impl::force_link()
    // if I use THIS function instead, no plugins get discovered
    // plugin_impl::aux::force_link()
};

pub fn main() {
    println!("listing all plugins");
    list_plugins();
}
