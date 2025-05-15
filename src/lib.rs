use godot::prelude::*;

// Import modules here
mod rust_example;
mod battle_manager;
mod player;

struct GdRust;

#[gdextension]
unsafe impl ExtensionLibrary for GdRust {}
