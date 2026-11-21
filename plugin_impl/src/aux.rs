/// Duplicate of the crate-root [`crate::force_link`], living in a submodule.
///
/// The binary's `PLUGIN_IMPORT` may reference this instead; see [`crate::force_link`].
pub fn force_link() {}
