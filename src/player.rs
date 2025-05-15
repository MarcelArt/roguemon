
use godot::{classes::{INode2D, Node2D}, prelude::*};

use crate::battle_manager::{BattleManager, TurnState};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Player {
    base: Base<Node2D>,

    battle_manager: Gd<BattleManager>,
}

#[godot_api]
impl INode2D for Player {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,

            battle_manager: BattleManager::new_alloc(),
        }
    }

    fn ready(&mut self) {
        // This is where you would put your initialization code
        godot_print!("Player is ready!");

        let battle_manager = self.base_mut().get_parent();
        
        if let None = battle_manager {
            return;
        }

        let battle_manager = battle_manager.unwrap();
        let bm: Gd<BattleManager> = battle_manager.cast();

        self.battle_manager = bm;

        godot_print!("Player's battle manager: {:?}", self.battle_manager);
    }

    fn process(&mut self, _delta: f64) {
        self.handle_keyboard_input();
    }

    // fn physics_process(&mut self, _delta: f64) {
    //     // This is where you would put your physics logic
    //     godot_print!("Hello from Rust in physics process!");
    // }    
}     

#[godot_api]
impl Player {
    #[func]
    fn handle_keyboard_input(&mut self) {
        let input = Input::singleton();
        if input.is_action_pressed("skill1") {
            godot_print!("Skill 1 casted!");
            let mut bm = self.battle_manager.bind_mut();
            godot_print!("Current turn state: {:?}", bm.get_turn_state());
            bm.set_turn_state(TurnState::Enemy);
        }
    }
}