mod systems;
mod resources;
mod components;
mod events;
mod plugin;


pub mod prelude {
    pub use super::plugin::TablePlugin;
    pub use super::events::*;
    pub use super::resources::*;
}