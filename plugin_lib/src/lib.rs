//! Shared plugin registry used by the `testinv` workspace.

/// A registered plugin.
///
/// Entries are published by `inventory::submit!` and iterated by [`list_plugins`].
pub struct Plugin {
    /// Human-readable label printed by [`list_plugins`].
    pub message: &'static str
}
inventory::collect!(Plugin);

/// Prints every registered [`Plugin`] (via `inventory::iter`).
pub fn list_plugins() {
    for plugin in inventory::iter::<Plugin> {
        println!("loaded plugin: {}", plugin.message);
    }
}
