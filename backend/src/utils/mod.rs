pub mod cli;
pub mod config;
pub mod converter;
pub mod database;
pub mod entity;
pub mod interface;
pub mod logger;
pub mod tui;

// Helper for serialize skip method.
pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
