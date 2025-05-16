use godot::prelude::*;

// Import modules here
mod rust_example;
mod battle_manager;
mod player;
mod mon;
mod skill;
mod enemy;

struct GdRust;

#[gdextension]
unsafe impl ExtensionLibrary for GdRust {}
