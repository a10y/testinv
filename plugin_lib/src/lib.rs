
pub struct Plugin {
    pub message: &'static str
}
inventory::collect!(Plugin);

pub fn list_plugins() {
    for plugin in inventory::iter::<Plugin> {
        println!("loaded plugin: {}", plugin.message);
    }
}
