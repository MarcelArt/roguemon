use godot::{classes::{INode2D, Node2D}, prelude::*};

use crate::battle_manager::BattleManager;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Enemy {
    base: Base<Node2D>,

    battle_manager: Gd<BattleManager>,
}

#[godot_api]
impl INode2D for Enemy {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,

            battle_manager: BattleManager::new_alloc(),
        }
    }

    fn ready(&mut self) {
    }

    fn process(&mut self, _delta: f64) {
    }
}     
        